use crate::hc_auth::{self, HcAuthConfig, HcAuthStatus};
use crate::{AppAuth, RuntimeConfig, RuntimeError, RuntimeResult, DEVICE_SEED_LAIR_TAG};
use crate::{AuthorizedAppClientsManager, ClientId};
use holochain::conductor::api::IssueAppAuthenticationTokenPayload;
use holochain::conductor::api::{AppAuthenticationTokenIssued, ZomeCallParamsSigned};
use holochain::{
    conductor::{
        api::{
            AdminInterfaceApi, AdminRequest, AdminResponse, AppInfo, AppInterfaceApi, AppRequest,
            AppResponse, CellInfo,
        },
        config::{ConductorConfig, KeystoreConfig, NetworkConfig},
        ConductorBuilder, ConductorHandle,
    },
    prelude::{AgentPubKey, AppStatus, CellId, InstallAppPayload, InstalledAppId, ZomeCallParams},
};
use holochain_keystore::MetaLairClient;
use holochain_types::network::HolochainTransportStats;
use holochain_types::signal::Signal;
use holochain_types::websocket::AllowedOrigins;
use lair_keystore_api::types::SharedLockedArray;
use log::{debug, error};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// Map of app ids to their associated app websocket & authentication
pub type AppAuths = Arc<RwLock<HashMap<InstalledAppId, AppAuth>>>;

/// Slim wrapper around holochain Conductor with calls wrapping AdminInterfaceApi requests
#[derive(Clone)]
pub struct Runtime {
    conductor: ConductorHandle,
    app_auths: AppAuths,
    authorized_app_clients: Arc<AuthorizedAppClientsManager>,

    // --- Phase 3: controllable boot / hc-auth / restart-keeping-lair ---
    /// The lair keystore client, spawned in-proc *before* the conductor and
    /// shared with it via `with_keystore`. Held so the conductor can be torn
    /// down and rebuilt (a hot restart) against the still-running keystore.
    lair_client: MetaLairClient,
    /// The device seed's own ed25519 key — a deterministic agent identity.
    /// holochain 0.6 mints a *random* key for installs with `agent_key: None`,
    /// so backup/restore can only reproduce identity if callers install under a
    /// key derived from the (backed-up) device seed. This is that key.
    device_agent_key: AgentPubKey,
    /// Passphrase used to (re)open lair, retained so a restart can rebuild the
    /// conductor without re-prompting the user.
    passphrase: SharedLockedArray,
    /// Conductor data root (also the lair root). The hc-auth flow persists its
    /// agent-key file here.
    data_root_path: PathBuf,
    /// hc-auth config when authenticated mode is in use; `None` in open mode.
    hc_auth_config: Option<HcAuthConfig>,
    hc_auth_status: Arc<RwLock<HcAuthStatus>>,
    hc_auth_agent_key: Arc<RwLock<Option<AgentPubKey>>>,
    hc_auth_raw_ed25519_b64url: Arc<RwLock<Option<String>>>,
}

/// Boot parameters for [`Runtime::new_with_boot_config`] — the controllable-boot
/// entry point used by the in-process Tauri plugin. Bundles the optional hc-auth
/// config and an optional pending device-seed import alongside the network config.
pub struct RuntimeBootConfig {
    pub data_root_path: PathBuf,
    pub network: NetworkConfig,
    /// If set, run the hc-auth flow before bringing the conductor up and inject
    /// the resulting material into the network config.
    pub hc_auth: Option<HcAuthConfig>,
    /// If set (must be 32 bytes), import this as the device seed *before* the
    /// auto-generate path, so the first install with `agent_key: None` derives
    /// this identity (used for agent-key restore). Ignored if a device seed
    /// already exists.
    pub pending_import_seed: Option<Vec<u8>>,
}

impl Runtime {
    /// Initialize and start a new Conductor from a [`RuntimeConfig`].
    ///
    /// This is the entry point used by the FFI layer (its network config is the
    /// serializable [`crate::RuntimeNetworkConfig`]).
    pub async fn new(
        passphrase: SharedLockedArray,
        runtime_config: RuntimeConfig,
    ) -> RuntimeResult<Self> {
        let data_root_path = runtime_config.data_root_path.clone();
        let network: NetworkConfig = runtime_config.network.into();
        Self::new_with_boot_config(
            passphrase,
            RuntimeBootConfig {
                data_root_path,
                network,
                hc_auth: None,
                pending_import_seed: None,
            },
        )
        .await
    }

    /// Initialize and start a new Conductor from a native holochain [`NetworkConfig`].
    ///
    /// This is the open-mode entry point for the in-process Tauri plugin (no
    /// hc-auth, no pending seed). For authenticated mode or seed restore, use
    /// [`Runtime::new_with_boot_config`].
    pub async fn new_with_network_config(
        passphrase: SharedLockedArray,
        data_root_path: PathBuf,
        network: NetworkConfig,
    ) -> RuntimeResult<Self> {
        Self::new_with_boot_config(
            passphrase,
            RuntimeBootConfig {
                data_root_path,
                network,
                hc_auth: None,
                pending_import_seed: None,
            },
        )
        .await
    }

    /// Controllable-boot constructor: spawn lair in-process, (optionally) run the
    /// hc-auth flow against it, then bring the conductor up on the same lair.
    ///
    /// Splitting lair from the conductor lets the hc-auth flow sign a challenge
    /// with the device/agent key and inject the resulting auth material into the
    /// `NetworkConfig` *before* the network starts — so authenticated bootstrap/
    /// relay work on the first (single) boot. The keystore is spawned at the same
    /// path holochain itself uses for `lair_root: None` (the data root), so a
    /// keystore created by an earlier internal-lair boot is reused as-is.
    pub async fn new_with_boot_config(
        passphrase: SharedLockedArray,
        boot: RuntimeBootConfig,
    ) -> RuntimeResult<Self> {
        let RuntimeBootConfig {
            data_root_path,
            mut network,
            hc_auth,
            pending_import_seed,
        } = boot;

        let res = rustls::crypto::aws_lc_rs::default_provider().install_default();
        if res.is_err() {
            error!("Failed to set default crypto provider for tls: {:?}", res);
        }

        // Phase (a): spawn lair in-proc at `<data_root>/lair-keystore-config.yaml`
        // (the location holochain uses when `lair_root` is `None`).
        let keystore_config_path = data_root_path.join("lair-keystore-config.yaml");
        let lair_client = holochain_keystore::lair_keystore::spawn_lair_keystore_in_proc(
            &keystore_config_path,
            passphrase.clone(),
        )
        .await
        .map_err(RuntimeError::Lair)?;

        // Ensure the device seed exists: import the pending seed as the device
        // seed if provided (agent-key restore), else generate a fresh exportable
        // one. Both are exportable so agent-key backup can read them later.
        let has_device_seed = lair_client
            .lair_client()
            .get_entry(DEVICE_SEED_LAIR_TAG.into())
            .await
            .is_ok();
        if !has_device_seed {
            match &pending_import_seed {
                Some(seed) => {
                    Self::import_seed_into_lair(&lair_client, seed, DEVICE_SEED_LAIR_TAG, true)
                        .await?;
                    log::info!("Imported pending seed as the device seed");
                }
                None => {
                    lair_client
                        .lair_client()
                        .new_seed(DEVICE_SEED_LAIR_TAG.into(), None, true)
                        .await
                        .map_err(RuntimeError::Lair)?;
                }
            }
        } else if pending_import_seed.is_some() {
            log::warn!("pending_import_seed provided but a device seed already exists; ignoring");
        }

        // The device seed's ed25519 key is unyt's deterministic agent identity
        // (see the field doc). Read it now that the seed is guaranteed to exist.
        let device_agent_key = Self::device_seed_agent_key(&lair_client).await?;

        // Phase between (a) and (b): run the hc-auth flow and inject the material.
        let (hc_auth_status, hc_auth_agent_key, hc_auth_raw_ed25519_b64url) = match &hc_auth {
            Some(cfg) => match hc_auth::perform_auth_flow(&lair_client, cfg, &data_root_path).await
            {
                Ok(result) => {
                    if let Some(material) = &result.auth_material {
                        if cfg.auth_bootstrap {
                            network.base64_auth_material_bootstrap = Some(material.clone());
                        }
                        if cfg.auth_relay {
                            network.base64_auth_material_relay = Some(material.clone());
                        }
                        log::info!(
                            "hc-auth: material injected (bootstrap={}, relay={})",
                            cfg.auth_bootstrap,
                            cfg.auth_relay
                        );
                    }
                    (
                        result.status,
                        Some(result.agent_key),
                        Some(result.raw_ed25519_b64url),
                    )
                }
                Err(e) => {
                    error!("hc-auth flow error: {e}");
                    (HcAuthStatus::Failed(format!("{e}")), None, None)
                }
            },
            None => (HcAuthStatus::Failed("Not configured".into()), None, None),
        };

        // Phase (b): build the conductor against our keystore + the (possibly
        // augmented) network. `with_keystore` short-circuits the conductor's own
        // lair spawn, so it uses exactly the keystore we just opened.
        let config = ConductorConfig {
            data_root_path: Some(data_root_path.clone().into()),
            keystore: KeystoreConfig::LairServerInProc { lair_root: None },
            network,
            ..Default::default()
        };
        let conductor = ConductorBuilder::default()
            .passphrase(Some(passphrase.clone()))
            .config(config)
            .with_keystore(lair_client.clone())
            .build()
            .await?;

        Ok(Self {
            conductor,
            app_auths: Arc::new(RwLock::new(HashMap::new())),
            authorized_app_clients: Arc::new(AuthorizedAppClientsManager::new(
                data_root_path.clone(),
            )?),
            lair_client,
            device_agent_key,
            passphrase,
            data_root_path,
            hc_auth_config: hc_auth,
            hc_auth_status: Arc::new(RwLock::new(hc_auth_status)),
            hc_auth_agent_key: Arc::new(RwLock::new(hc_auth_agent_key)),
            hc_auth_raw_ed25519_b64url: Arc::new(RwLock::new(hc_auth_raw_ed25519_b64url)),
        })
    }

    /// Box-encrypt `seed` (32 bytes) to a fresh lair recipient entry and import it
    /// under `tag`. Shared by [`Runtime::import_key_seed`] and the device-seed
    /// import path. Mirrors lair's `import_seed` contract: a local sender keypair
    /// encrypts to a lair-held recipient, which lair then decrypts and stores.
    async fn import_seed_into_lair(
        keystore: &MetaLairClient,
        seed: &[u8],
        tag: &str,
        exportable: bool,
    ) -> RuntimeResult<AgentPubKey> {
        if seed.len() != 32 {
            return Err(RuntimeError::AgentSeed(format!(
                "seed must be exactly 32 bytes, got {}",
                seed.len()
            )));
        }
        let client = keystore.lair_client();

        // Local sender keypair (never enters lair).
        let mut sender_pk = [0u8; sodoken::crypto_box::XSALSA_PUBLICKEYBYTES];
        let mut sender_sk =
            sodoken::SizedLockedArray::new().map_err(|e| RuntimeError::Lair(e.into()))?;
        sodoken::crypto_box::xsalsa_keypair(&mut sender_pk, &mut sender_sk.lock())
            .map_err(|e| RuntimeError::Lair(e.into()))?;

        // Fresh lair recipient entry; lair holds its secret to decrypt the import.
        let recipient_info = client
            .new_seed(uuid::Uuid::new_v4().to_string().into(), None, false)
            .await
            .map_err(RuntimeError::Lair)?;
        let recipient_pk = recipient_info.x25519_pub_key;

        let mut nonce = [0u8; sodoken::crypto_box::XSALSA_NONCEBYTES];
        sodoken::random::randombytes_buf(&mut nonce).map_err(|e| RuntimeError::Lair(e.into()))?;
        let mut cipher = vec![0u8; 32 + sodoken::crypto_box::XSALSA_MACBYTES];
        let mut seed_arr = [0u8; 32];
        seed_arr.copy_from_slice(seed);
        sodoken::crypto_box::xsalsa_easy(
            &mut cipher,
            &seed_arr,
            &nonce,
            &recipient_pk,
            &sender_sk.lock(),
        )
        .map_err(|e| RuntimeError::Lair(e.into()))?;

        let seed_info = client
            .import_seed(
                sender_pk.into(),
                recipient_pk,
                None,
                nonce,
                cipher.into(),
                tag.into(),
                exportable,
            )
            .await
            .map_err(RuntimeError::Lair)?;

        Ok(AgentPubKey::from_raw_32(
            seed_info.ed25519_pub_key.0.to_vec(),
        ))
    }

    /// Stop the Conductor
    ///
    /// This is *NOT* fully implemented by holochain: kitsune tasks will continue to run.
    pub async fn stop(&self) -> RuntimeResult<()> {
        self.conductor
            .clone()
            .shutdown()
            .await
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?;

        Ok(())
    }

    pub async fn install_app(&self, payload: InstallAppPayload) -> RuntimeResult<AppInfo> {
        // Install against the conductor handle directly rather than through
        // `AdminInterfaceApi::handle_request`, which flattens every error into a
        // print-only `ExternalApiWireError::InternalError(String)` (holochain TODO
        // B-01506). Going direct preserves the typed `ConductorError` — surfaced as
        // `RuntimeError::Conductor` — so callers can match on the actual failure
        // instead of grepping a Debug string. We lose `handle_request`'s implicit
        // `check_running` guard, so run it here for fast-fail + parity with the
        // sibling `req_admin_api` calls.
        self.conductor.check_running()?;
        let app = self.conductor.clone().install_app_bundle(payload).await?;
        // Mirror the conductor's admin `InstallApp` handler: resolve the app's DNA
        // definitions and build the same `AppInfo` it would have returned.
        let dna_definitions = self.conductor.get_dna_definitions(&app).await?;
        Ok(AppInfo::from_installed_app(&app, &dna_definitions))
    }

    pub async fn uninstall_app(&self, installed_app_id: String) -> RuntimeResult<()> {
        let response = self
            .req_admin_api(AdminRequest::UninstallApp {
                installed_app_id,
                force: false,
            })
            .await?;
        match response {
            AdminResponse::AppUninstalled => Ok(()),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    pub async fn enable_app(&self, installed_app_id: InstalledAppId) -> RuntimeResult<AppInfo> {
        let response = self
            .req_admin_api(AdminRequest::EnableApp { installed_app_id })
            .await?;
        match response {
            AdminResponse::AppEnabled(app) => Ok(app),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    pub async fn disable_app(&self, installed_app_id: InstalledAppId) -> RuntimeResult<()> {
        let response = self
            .req_admin_api(AdminRequest::DisableApp { installed_app_id })
            .await?;
        match response {
            AdminResponse::AppDisabled => Ok(()),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    pub async fn list_apps(&self) -> RuntimeResult<Vec<AppInfo>> {
        let response = self
            .req_admin_api(AdminRequest::ListApps {
                status_filter: None,
            })
            .await?;
        match response {
            AdminResponse::AppsListed(apps) => Ok(apps),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    /// Dump the conductor's networking stats.
    ///
    /// Returns the typed [`HolochainTransportStats`] (not a stringified dump) so callers
    /// can read fields like `transport_stats.backend` directly. This is the
    /// in-process equivalent of an admin `DumpNetworkStats` request.
    pub async fn dump_network_stats(&self) -> RuntimeResult<HolochainTransportStats> {
        let response = self.req_admin_api(AdminRequest::DumpNetworkStats).await?;
        match response {
            AdminResponse::NetworkStatsDumped(stats) => Ok(stats),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    pub async fn is_app_installed(&self, installed_app_id: InstalledAppId) -> RuntimeResult<bool> {
        Ok(self
            .list_apps()
            .await?
            .into_iter()
            .any(|app_info| app_info.installed_app_id == installed_app_id))
    }

    /// Generate a fresh agent key in lair (no app install). Used for the hc-auth
    /// pre-registration / standalone key-generation path; open-mode installs
    /// derive their key implicitly, so they don't need this.
    pub async fn generate_agent_pub_key(&self) -> RuntimeResult<AgentPubKey> {
        let response = self
            .req_admin_api(AdminRequest::GenerateAgentPubKey)
            .await?;
        match response {
            AdminResponse::AgentPubKeyGenerated(key) => Ok(key),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    pub async fn import_key_seed(&self, seed: [u8; 32]) -> RuntimeResult<AgentPubKey> {
        // Imported under a fresh (uuid) tag, non-exportable — matching the prior
        // behavior of this entry point. (The device-seed restore path imports
        // under `DEVICE_SEED_LAIR_TAG`, exportable, during boot.)
        Self::import_seed_into_lair(
            &self.lair_client,
            &seed,
            &uuid::Uuid::new_v4().to_string(),
            false,
        )
        .await
    }

    // --- Phase 3 accessors / restart / seed export ---

    /// Admin websocket port. In the in-process model there is no admin websocket,
    /// so this is a stub `0`; unyt only reads it for logging.
    pub fn admin_port(&self) -> u16 {
        0
    }

    /// Current hc-auth status (populated by the boot/restart auth flow).
    pub fn hc_auth_status(&self) -> HcAuthStatus {
        self.hc_auth_status.read().unwrap().clone()
    }

    /// The hc-auth agent key, if the auth flow has run.
    pub fn hc_auth_agent_key(&self) -> Option<AgentPubKey> {
        self.hc_auth_agent_key.read().unwrap().clone()
    }

    /// The hc-auth agent key as raw-Ed25519 URL-safe base64 (no padding).
    pub fn hc_auth_raw_ed25519_b64url(&self) -> Option<String> {
        self.hc_auth_raw_ed25519_b64url.read().unwrap().clone()
    }

    /// Whether hc-auth (authenticated mode) is configured.
    pub fn is_hc_auth_configured(&self) -> bool {
        self.hc_auth_config.is_some()
    }

    /// unyt's deterministic agent identity: the device seed's ed25519 key.
    /// Installing an app under this key (instead of letting the conductor mint a
    /// random key) is what makes agent-key backup/restore reproduce identity —
    /// exporting then re-importing the device seed yields this same key.
    pub fn device_agent_key(&self) -> AgentPubKey {
        self.device_agent_key.clone()
    }

    /// Read the ed25519 agent key of the device seed (`DEVICE_SEED_LAIR_TAG`),
    /// which must already exist. Stable for a given seed.
    async fn device_seed_agent_key(keystore: &MetaLairClient) -> RuntimeResult<AgentPubKey> {
        use lair_keystore_api::lair_store::LairEntryInfo;
        let entry = keystore
            .lair_client()
            .get_entry(DEVICE_SEED_LAIR_TAG.into())
            .await
            .map_err(RuntimeError::Lair)?;
        match entry {
            LairEntryInfo::Seed { seed_info, .. }
            | LairEntryInfo::DeepLockedSeed { seed_info, .. } => Ok(AgentPubKey::from_raw_32(
                seed_info.ed25519_pub_key.0.to_vec(),
            )),
            _ => Err(RuntimeError::AgentSeed(
                "device seed entry is not a seed".into(),
            )),
        }
    }

    /// Export the raw 32-byte **device seed**, decrypted, for agent-key backup.
    ///
    /// Asks lair to box-encrypt the device seed to a fresh local recipient
    /// keypair (the inverse of [`Runtime::import_key_seed`]), then decrypts it
    /// locally. The device seed is the open-mode identity; note that in
    /// authenticated mode the hc-auth agent key is a separate lair key, so this
    /// backs up the device identity, not the hc-auth key.
    pub async fn export_agent_seed(&self) -> RuntimeResult<[u8; 32]> {
        let client = self.lair_client.lair_client();

        // Local recipient keypair — we hold the secret to decrypt the export.
        let mut recipient_pk = [0u8; sodoken::crypto_box::XSALSA_PUBLICKEYBYTES];
        let mut recipient_sk =
            sodoken::SizedLockedArray::new().map_err(|e| RuntimeError::Lair(e.into()))?;
        sodoken::crypto_box::xsalsa_keypair(&mut recipient_pk, &mut recipient_sk.lock())
            .map_err(|e| RuntimeError::Lair(e.into()))?;

        // Sender must be a lair-held key (lair encrypts the export with its
        // secret). Create a throwaway lair seed to act as the sender.
        let sender_info = client
            .new_seed(uuid::Uuid::new_v4().to_string().into(), None, false)
            .await
            .map_err(RuntimeError::Lair)?;
        let sender_pk = sender_info.x25519_pub_key;

        let (nonce, cipher) = client
            .export_seed_by_tag(
                DEVICE_SEED_LAIR_TAG.into(),
                sender_pk.clone(),
                recipient_pk.into(),
                None,
            )
            .await
            .map_err(RuntimeError::Lair)?;

        if cipher.len() != 32 + sodoken::crypto_box::XSALSA_MACBYTES {
            return Err(RuntimeError::AgentSeed(format!(
                "unexpected exported cipher length {}",
                cipher.len()
            )));
        }
        let mut seed = [0u8; 32];
        sodoken::crypto_box::xsalsa_open_easy(
            &mut seed,
            &cipher,
            &nonce,
            &sender_pk,
            &recipient_sk.lock(),
        )
        .map_err(|e| RuntimeError::AgentSeed(format!("failed to decrypt exported seed: {e}")))?;
        Ok(seed)
    }

    /// Shut down only the conductor, leaving the in-proc lair keystore running.
    ///
    /// Enabled apps are disabled first so their cells leave the network cleanly.
    /// The keystore stays up (it's our separate in-proc client), so a subsequent
    /// [`Runtime::restart_with_hc_auth`] can rebuild the conductor on it.
    pub async fn stop_conductor_only(&self) -> RuntimeResult<()> {
        let apps = self.list_apps().await?;
        for app in apps {
            if matches!(app.status, AppStatus::Enabled) {
                if let Err(e) = self.disable_app(app.installed_app_id.clone()).await {
                    error!(
                        "Error disabling app {} on shutdown: {e:?}",
                        app.installed_app_id
                    );
                }
            }
        }
        self.conductor
            .clone()
            .shutdown()
            .await
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?;
        log::info!("Conductor shut down (lair still running)");
        Ok(())
    }

    /// Restart the conductor with fresh hc-auth material, keeping lair running.
    ///
    /// Re-runs the auth flow against the still-running keystore, injects the new
    /// material into `network`, and rebuilds the conductor on the same lair.
    /// Returns a NEW [`Runtime`] sharing this one's keystore/passphrase. Errors if
    /// hc-auth was not configured at boot.
    pub async fn restart_with_hc_auth(
        &self,
        mut network: NetworkConfig,
        add_auth_material_to_bootstrap: bool,
        add_auth_material_to_relay: bool,
    ) -> RuntimeResult<Runtime> {
        let cfg = self
            .hc_auth_config
            .clone()
            .ok_or_else(|| RuntimeError::HcAuth("hc-auth not configured".into()))?;

        log::info!("hc-auth restart: shutting down conductor (lair stays up)");
        self.stop_conductor_only().await?;

        log::info!("hc-auth restart: generating fresh auth material");
        let result =
            hc_auth::perform_auth_flow(&self.lair_client, &cfg, &self.data_root_path).await?;
        if let Some(material) = &result.auth_material {
            if add_auth_material_to_bootstrap {
                network.base64_auth_material_bootstrap = Some(material.clone());
            }
            if add_auth_material_to_relay {
                network.base64_auth_material_relay = Some(material.clone());
            }
        }

        let config = ConductorConfig {
            data_root_path: Some(self.data_root_path.clone().into()),
            keystore: KeystoreConfig::LairServerInProc { lair_root: None },
            network,
            ..Default::default()
        };
        let conductor = ConductorBuilder::default()
            .passphrase(Some(self.passphrase.clone()))
            .config(config)
            .with_keystore(self.lair_client.clone())
            .build()
            .await?;
        log::info!("hc-auth restart: conductor restarted");

        Ok(Runtime {
            conductor,
            app_auths: Arc::new(RwLock::new(HashMap::new())),
            authorized_app_clients: Arc::new(AuthorizedAppClientsManager::new(
                self.data_root_path.clone(),
            )?),
            lair_client: self.lair_client.clone(),
            device_agent_key: self.device_agent_key.clone(),
            passphrase: self.passphrase.clone(),
            data_root_path: self.data_root_path.clone(),
            hc_auth_config: Some(cfg),
            hc_auth_status: Arc::new(RwLock::new(result.status)),
            hc_auth_agent_key: Arc::new(RwLock::new(Some(result.agent_key))),
            hc_auth_raw_ed25519_b64url: Arc::new(RwLock::new(Some(result.raw_ed25519_b64url))),
        })
    }

    pub async fn sign_zome_call(
        &self,
        zome_call_params: ZomeCallParams,
    ) -> RuntimeResult<ZomeCallParamsSigned> {
        let (bytes, hash) = zome_call_params
            .serialize_and_hash()
            .map_err(|e| RuntimeError::ZomeCallParamsInvalid(e.to_string()))?;
        let signer_key: [u8; 32] = zome_call_params
            .provenance
            .get_raw_32()
            .try_into()
            .map_err(|_| RuntimeError::ZomeCallParamsInvalid("Invalid provenance".to_string()))?;
        let signature = self
            .conductor
            .keystore()
            .lair_client()
            .sign_by_pub_key(signer_key.into(), None, hash.into())
            .await
            .map_err(RuntimeError::Lair)?;

        Ok(ZomeCallParamsSigned {
            bytes: bytes.into(),
            signature: (*signature.0).into(),
        })
    }

    pub async fn ensure_app_websocket(
        &self,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<AppAuth> {
        let app_auths = self.app_auths.read().unwrap().clone();
        match app_auths.get(&installed_app_id) {
            Some(app_websocket) => Ok(app_websocket.clone()),
            None => {
                let authentication = self
                    .issue_app_authentication_token(IssueAppAuthenticationTokenPayload {
                        installed_app_id: installed_app_id.clone(),
                        expiry_seconds: 0,
                        single_use: false,
                    })
                    .await?;
                let port = self
                    .attach_app_interface(None, AllowedOrigins::Any, Some(installed_app_id.clone()))
                    .await?;
                let app_auth = AppAuth {
                    authentication,
                    port,
                };

                let mut app_auths = self.app_auths.write().unwrap();
                app_auths.insert(installed_app_id, app_auth.clone());

                Ok(app_auth)
            }
        }
    }

    /// Full process to setup an app
    ///
    /// Check if app is installed, if not install it, then optionally enable it.
    /// Then ensure there is an app websocket and authentication for it.
    ///
    /// If an app is already installed, it will not be enabled. It is only enabled after a successful install.
    /// The reasoning is that if an app is disabled after that point,
    /// it is assumed to have been manually disabled in the admin interface, which we don't want to override.
    pub async fn setup_app(
        &self,
        payload: InstallAppPayload,
        enable_after_install: bool,
    ) -> RuntimeResult<AppAuth> {
        // This is a temporary workaround because we cannot clone AppBundleSource,
        // which is needed to read the actual app name from the manifest
        // See https://github.com/holochain/holochain/pull/4882
        let installed_app_id = payload
            .installed_app_id
            .clone()
            .ok_or(RuntimeError::InstalledAppIdNotSpecified)?;

        if self.is_app_installed(installed_app_id.clone()).await? {
            debug!(
                "App {} is already installed, skipping install and enable",
                installed_app_id.clone()
            );
        } else {
            let _ = self.install_app(payload).await?;
            if enable_after_install {
                let _ = self.enable_app(installed_app_id.clone()).await?;
            }
        }

        self.ensure_app_websocket(installed_app_id).await
    }

    pub fn authorize_app_client(
        &self,
        client_uid: ClientId,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<()> {
        self.authorized_app_clients
            .authorize(client_uid, installed_app_id)
    }

    pub fn is_app_client_authorized(
        &self,
        client_uid: ClientId,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<bool> {
        self.authorized_app_clients
            .is_authorized(client_uid, installed_app_id)
    }

    /// Dispatch an [`AppRequest`] for `installed_app_id` against the in-process
    /// conductor and return the [`AppResponse`].
    ///
    /// This is the in-process equivalent of what the conductor's app websocket
    /// interface does for each connected client: it routes the full App API
    /// (`AppInfo`, `CallZome`, `CreateCloneCell`, `DumpNetwork*`, ...) without a
    /// loopback websocket. It is the App-API twin of the private
    /// [`Self::req_admin_api`] and lets a Tauri command serve `@holochain/client`
    /// calls directly.
    ///
    /// The caller is responsible for scoping `installed_app_id` to what the
    /// requester is allowed to access — the app websocket path uses a per-app
    /// auth token for this; the in-process path must bind it some other way
    /// (e.g. the calling window).
    pub async fn handle_app_request(
        &self,
        installed_app_id: InstalledAppId,
        request: AppRequest,
    ) -> RuntimeResult<AppResponse> {
        Ok(AppInterfaceApi::new(self.conductor.clone())
            .handle_request(installed_app_id, Ok(request))
            .await?)
    }

    /// Subscribe to the signal stream for `installed_app_id`. Each [`Signal`]
    /// the app's cells emit (e.g. from a zome's `post_commit`) is delivered to
    /// the returned receiver.
    ///
    /// This is what lets a Tauri plugin forward signals to a webview without an
    /// app websocket. The conductor's broadcast is keyed by app, but its only
    /// public accessor is keyed by cell, so this resolves one of the app's
    /// provisioned cells first.
    pub async fn subscribe_to_app_signals(
        &self,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<broadcast::Receiver<Signal>> {
        let cell_id = self.first_provisioned_cell_id(&installed_app_id).await?;
        let sender = self.conductor.get_signal_tx(&cell_id).await?;
        Ok(sender.subscribe())
    }

    /// Find a provisioned cell of `installed_app_id`, used to locate the app's
    /// signal broadcast channel.
    async fn first_provisioned_cell_id(
        &self,
        installed_app_id: &InstalledAppId,
    ) -> RuntimeResult<CellId> {
        let app = self
            .list_apps()
            .await?
            .into_iter()
            .find(|app| &app.installed_app_id == installed_app_id)
            .ok_or_else(|| {
                RuntimeError::InvalidArguments(format!("app not installed: {installed_app_id}"))
            })?;
        app.cell_info
            .values()
            .flatten()
            .find_map(|cell_info| match cell_info {
                CellInfo::Provisioned(cell) => Some(cell.cell_id.clone()),
                _ => None,
            })
            .ok_or_else(|| {
                RuntimeError::InvalidArguments(format!(
                    "app has no provisioned cell: {installed_app_id}"
                ))
            })
    }

    async fn req_admin_api(&self, request: AdminRequest) -> RuntimeResult<AdminResponse> {
        Ok(AdminInterfaceApi::new(self.conductor.clone())
            .handle_request(Ok(request))
            .await?)
    }

    async fn issue_app_authentication_token(
        &self,
        payload: IssueAppAuthenticationTokenPayload,
    ) -> RuntimeResult<AppAuthenticationTokenIssued> {
        let response = self
            .req_admin_api(AdminRequest::IssueAppAuthenticationToken(payload))
            .await?;
        match response {
            AdminResponse::AppAuthenticationTokenIssued(auth) => Ok(auth),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }

    async fn attach_app_interface(
        &self,
        port: Option<u16>,
        allowed_origins: AllowedOrigins,
        installed_app_id: Option<InstalledAppId>,
    ) -> RuntimeResult<u16> {
        let response = self
            .req_admin_api(AdminRequest::AttachAppInterface {
                port,
                allowed_origins,
                installed_app_id,
                danger_bind_addr: None,
            })
            .await?;
        match response {
            AdminResponse::AppInterfaceAttached { port } => Ok(port),
            fail => Err(RuntimeError::AdminApiBadResponse(Box::new(fail))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{ConductorError, RuntimeNetworkConfig};

    use super::*;
    use holochain::conductor::api::CellInfo::Provisioned;
    use holochain::conductor::api::ProvisionedCell;
    use holochain::conductor::config::KeystoreConfig;
    use holochain_types::prelude::AppBundleSource;
    use holochain_types::prelude::AppStatus;
    use holochain_types::prelude::DisabledAppReason;
    use holochain_types::prelude::ExternIO;
    use holochain_types::prelude::Link;
    use holochain_types::prelude::Nonce256Bits;
    use holochain_types::prelude::Timestamp;

    use sodoken::LockedArray;
    use std::sync::Mutex;
    use std::time::Duration;
    use tempfile::TempDir;
    use url2::Url2;
    use uuid::Uuid;

    const HAPP_FIXTURE: &[u8] = include_bytes!("../fixtures/forum.happ");

    async fn install_happ_fixture(runtime: Runtime, app_id: &str) -> AppInfo {
        runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: None,
                installed_app_id: Some(app_id.into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            })
            .await
            .unwrap()
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_new_runtime() {
        let tmp_dir = TempDir::new().unwrap();
        let bootstrap_url = Url2::try_parse("https://bootstrap.com").unwrap();
        let relay_url = Url2::try_parse("https://relay.com").unwrap();

        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig {
                    bootstrap_url: bootstrap_url.clone(),
                    relay_url: relay_url.clone(),
                },
            },
        )
        .await
        .unwrap();

        assert_eq!(
            runtime
                .conductor
                .config
                .data_root_path
                .clone()
                .unwrap()
                .as_path(),
            tmp_dir.path()
        );
        assert_eq!(
            runtime.conductor.config.keystore,
            KeystoreConfig::LairServerInProc { lair_root: None }
        );
        assert_eq!(
            runtime.conductor.config.network.bootstrap_url.clone(),
            bootstrap_url
        );
        assert_eq!(
            runtime.conductor.config.network.relay_url.clone(),
            relay_url
        );

        let res = AdminInterfaceApi::new(runtime.conductor)
            .handle_request(Ok(AdminRequest::DumpConductorState))
            .await;
        assert!(res.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_stop() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        runtime.stop().await.unwrap();

        let res = AdminInterfaceApi::new(runtime.conductor)
            .handle_request(Ok(AdminRequest::DumpConductorState))
            .await;
        assert!(res.is_err());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_dump_network_stats() {
        let tmp_dir = TempDir::new().unwrap();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().to_path_buf(),
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        // Returns the typed stats struct, so `transport_stats.backend` is readable
        // directly (the field unyt's About dialog reads).
        let stats = runtime.dump_network_stats().await.unwrap();
        assert!(!stats.transport_stats.backend.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_install_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: None,
                installed_app_id: Some("my-app-1".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            })
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
    }

    /// Re-installing an already-present `installed_app_id` surfaces the conductor's
    /// own typed `ConductorError::AppAlreadyInstalled` (carrying the app id) through
    /// `RuntimeError::Conductor` — so a consumer can detect the benign case with a
    /// `matches!` instead of a `format!("{e:?}").contains(..)` Debug-string grep.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_install_app_already_installed() {
        let tmp_dir = TempDir::new().unwrap();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        // First install of this id succeeds.
        let original = install_happ_fixture(runtime.clone(), "my-app-1").await;

        // Re-installing the same id fails with the typed conductor error. A fresh
        // network_seed proves the conflict is keyed on the app id, not the bundle.
        let err = runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: None,
                installed_app_id: Some("my-app-1".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            })
            .await
            .expect_err("re-installing an existing app id must fail");

        assert!(
            matches!(&err, RuntimeError::Conductor(ce)
                if matches!(&**ce, ConductorError::AppAlreadyInstalled(id) if id == "my-app-1")),
            "expected RuntimeError::Conductor(ConductorError::AppAlreadyInstalled(\"my-app-1\")), got: {err:?}"
        );

        // The failed re-install left the original app untouched — not just the same
        // count, but the same agent key and cells. Had the fresh-seed re-install
        // wrongly replaced it, the surviving app's provisioned DNA hash would differ.
        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].installed_app_id, original.installed_app_id);
        assert_eq!(apps[0].agent_pub_key, original.agent_pub_key);
        assert_eq!(apps[0].cell_info, original.cell_info);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_import_key_seed() {
        async fn make_runtime() -> (Runtime, TempDir) {
            let tmp_dir = TempDir::new().unwrap();
            let runtime = Runtime::new(
                Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
                RuntimeConfig {
                    data_root_path: tmp_dir.path().into(),
                    network: RuntimeNetworkConfig::default(),
                },
            )
            .await
            .unwrap();
            (runtime, tmp_dir)
        }

        let seed = [7u8; 32];

        // Importing a seed returns a 32-byte agent key.
        let (rt1, _d1) = make_runtime().await;
        let agent_a = rt1.import_key_seed(seed).await.unwrap();
        assert_eq!(agent_a.get_raw_32().len(), 32);

        // Recovery property: importing the same seed into a *fresh* keystore
        // reproduces the same agent key — this is what makes seed-based identity
        // recovery work across devices/reinstalls.
        let (rt2, _d2) = make_runtime().await;
        let agent_b = rt2.import_key_seed(seed).await.unwrap();
        assert_eq!(agent_a, agent_b);

        // A different seed yields a different agent key.
        let agent_c = rt2.import_key_seed([9u8; 32]).await.unwrap();
        assert_ne!(agent_a, agent_c);
    }

    fn empty_passphrase() -> SharedLockedArray {
        Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0])))
    }

    async fn boot_runtime(dir: &TempDir, pending_import_seed: Option<Vec<u8>>) -> Runtime {
        Runtime::new_with_boot_config(
            empty_passphrase(),
            RuntimeBootConfig {
                data_root_path: dir.path().into(),
                network: RuntimeNetworkConfig::default().into(),
                hc_auth: None,
                pending_import_seed,
            },
        )
        .await
        .unwrap()
    }

    /// The seed-export crypto (`export_seed_by_tag` + local box-decrypt) must be
    /// the exact inverse of the pending-seed import: a device seed exported from
    /// one keystore, imported as the device seed of a fresh keystore, and
    /// re-exported, must come back byte-for-byte identical.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_export_agent_seed_round_trips() {
        // Ground truth: import a KNOWN, non-uniform seed as the device seed and
        // assert the *exact* bytes come back out. This pins export to a concrete
        // value (not merely "the inverse of import"), so a compensating
        // export/import bug can't slip through. Non-uniform so a stuck/zeroed
        // buffer would be caught.
        let known: [u8; 32] = std::array::from_fn(|i| (i as u8).wrapping_mul(7).wrapping_add(3));
        let tmp0 = TempDir::new().unwrap();
        let rt0 = boot_runtime(&tmp0, Some(known.to_vec())).await;
        let exported = rt0.export_agent_seed().await.unwrap();
        assert_eq!(
            exported, known,
            "exported device seed must equal the imported known seed (byte-exact)"
        );

        // And a random round-trip across two fresh keystores.
        let tmp1 = TempDir::new().unwrap();
        let rt1 = boot_runtime(&tmp1, None).await;
        let seed1 = rt1.export_agent_seed().await.unwrap();
        assert_eq!(seed1.len(), 32);

        let tmp2 = TempDir::new().unwrap();
        let rt2 = boot_runtime(&tmp2, Some(seed1.to_vec())).await;
        let seed2 = rt2.export_agent_seed().await.unwrap();

        assert_eq!(
            seed1, seed2,
            "an imported device seed must export back identically"
        );
    }

    /// Agent-key restore: install under the device agent key (as unyt does),
    /// export the device seed, then in a fresh keystore seeded with that device
    /// seed confirm the device agent key — and thus the installable identity —
    /// is reproduced. This is the backup/restore round-trip.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_agent_key_restore_reproduces_identity() {
        // rt1: install under the device agent key, then export the device seed.
        let tmp1 = TempDir::new().unwrap();
        let rt1 = boot_runtime(&tmp1, None).await;
        let key1 = rt1.device_agent_key();
        let app1 = rt1
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: Some(key1.clone()),
                installed_app_id: Some("app-1".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            })
            .await
            .unwrap();
        assert_eq!(
            app1.agent_pub_key, key1,
            "install should honor the supplied device agent key"
        );
        let seed = rt1.export_agent_seed().await.unwrap();

        // rt2: restoring the device seed must reproduce the same device agent key.
        let tmp2 = TempDir::new().unwrap();
        let rt2 = boot_runtime(&tmp2, Some(seed.to_vec())).await;
        let key2 = rt2.device_agent_key();

        assert_eq!(
            key1, key2,
            "restoring the device seed must reproduce the device agent key"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_install_app_with_agent_key() {
        let tmp_dir = TempDir::new().unwrap();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        // Import a seed, then install the app under that explicit agent key.
        let agent_key = runtime.import_key_seed([3u8; 32]).await.unwrap();
        let app_info = runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: Some(agent_key.clone()),
                installed_app_id: Some("my-app-1".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            })
            .await
            .unwrap();

        // The installed app uses the imported agent key rather than a fresh one.
        assert_eq!(app_info.agent_pub_key, agent_key);
    }

    /// Installing under an agent key that lives in a *different* lair (so the
    /// local keystore can't sign for it) fails up front with
    /// `AgentKeyNotInKeystore` — since holochain 0.7 the conductor checks the
    /// key before attempting genesis rather than failing genesis with an empty
    /// source-chain read. This is the exact failure a stale persisted agent key
    /// produces in a consumer app.
    #[tokio::test(flavor = "multi_thread")]
    async fn test_install_with_foreign_key_fails_genesis() {
        let tmp1 = TempDir::new().unwrap();
        let rt1 = boot_runtime(&tmp1, None).await;

        // A key from a second, independent keystore — not signable in rt1.
        let tmp2 = TempDir::new().unwrap();
        let rt2 = boot_runtime(&tmp2, None).await;
        let foreign_key = rt2.device_agent_key();
        assert_ne!(foreign_key, rt1.device_agent_key());

        let result = rt1
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: Some(foreign_key),
                installed_app_id: Some("forum-foreign".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            })
            .await;

        let msg = format!(
            "{:?}",
            result.expect_err("install under a non-local agent key must fail")
        );
        assert!(
            msg.contains("AgentKeyNotInKeystore"),
            "expected AgentKeyNotInKeystore, got: {msg}"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_uninstall_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;

        let res = runtime.uninstall_app("my-app-1".into()).await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 0);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_enable_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;

        let res = runtime.enable_app("my-app-1".into()).await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps.first().unwrap().status, AppStatus::Enabled);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_disable_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;
        runtime.enable_app("my-app-1".into()).await.unwrap();

        runtime.disable_app("my-app-1".into()).await.unwrap();

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
        assert!(matches!(
            apps.first().unwrap().status,
            AppStatus::Disabled(DisabledAppReason::User)
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_apps() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;
        install_happ_fixture(runtime.clone(), "my-app-2").await;

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 2);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_is_app_installed() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let is_installed = runtime.is_app_installed("my-app-1".into()).await.unwrap();
        assert!(!is_installed);

        install_happ_fixture(runtime.clone(), "my-app-1").await;

        let is_installed = runtime.is_app_installed("my-app-1".into()).await.unwrap();
        assert!(is_installed);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn sign_zome_call() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let app_info = install_happ_fixture(runtime.clone(), "my-app-1").await;
        let Provisioned(ProvisionedCell { cell_id, .. }) =
            app_info.cell_info.get("forum").unwrap().first().unwrap()
        else {
            panic!("App Info has no CellId")
        };

        let res = runtime
            .sign_zome_call(ZomeCallParams {
                provenance: cell_id.agent_pubkey().clone(),
                cell_id: cell_id.clone(),
                zome_name: "forum".into(),
                fn_name: "get_all_posts".into(),
                cap_secret: None,
                payload: vec![].into(),
                nonce: Nonce256Bits::from([0; 32]),
                expires_at: Timestamp(Timestamp::now().as_micros() + 100000),
            })
            .await;
        assert!(res.is_ok())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_app_request_app_info() {
        let tmp_dir = TempDir::new().unwrap();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;
        runtime.enable_app("my-app-1".into()).await.unwrap();

        // Dispatching AppRequest::AppInfo in-process returns the same AppInfo the
        // app websocket interface would serve, with no loopback socket involved.
        let resp = runtime
            .handle_app_request("my-app-1".into(), AppRequest::AppInfo)
            .await
            .unwrap();
        let AppResponse::AppInfo(Some(app_info)) = resp else {
            panic!("expected AppResponse::AppInfo(Some(_)), got {resp:?}");
        };
        assert_eq!(app_info.installed_app_id, "my-app-1");
        assert_eq!(app_info.status, AppStatus::Enabled);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_handle_app_request_call_zome() {
        let tmp_dir = TempDir::new().unwrap();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        let app_info = install_happ_fixture(runtime.clone(), "my-app-1").await;
        runtime.enable_app("my-app-1".into()).await.unwrap();

        // Role name is "forum"; the coordinator zome inside it is "posts" (see
        // dnas/forum/workdir/dna.yaml and the example UI's AllPosts.svelte).
        let Provisioned(ProvisionedCell { cell_id, .. }) =
            app_info.cell_info.get("forum").unwrap().first().unwrap()
        else {
            panic!("App Info has no CellId")
        };

        // Sign a real zome call the same way the webview signer does, then
        // dispatch it through the in-process app API (AppRequest::CallZome)
        // rather than the websocket. get_all_posts takes no arguments, so the
        // payload is an encoded unit; an empty byte vec would fail to decode.
        let signed = runtime
            .sign_zome_call(ZomeCallParams {
                provenance: cell_id.agent_pubkey().clone(),
                cell_id: cell_id.clone(),
                zome_name: "posts".into(),
                fn_name: "get_all_posts".into(),
                cap_secret: None,
                payload: ExternIO::encode(()).unwrap(),
                nonce: Nonce256Bits::from([0; 32]),
                expires_at: Timestamp(Timestamp::now().as_micros() + 60_000_000),
            })
            .await
            .unwrap();

        let resp = runtime
            .handle_app_request("my-app-1".into(), AppRequest::CallZome(Box::new(signed)))
            .await
            .unwrap();

        // A fresh forum app has no posts: the call round-trips to an empty list.
        let AppResponse::ZomeCalled(io) = resp else {
            panic!("expected AppResponse::ZomeCalled, got {resp:?}");
        };
        let posts: Vec<Link> = io.decode().unwrap();
        assert!(posts.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_subscribe_to_app_signals() {
        let tmp_dir = TempDir::new().unwrap();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();
        let app_info = install_happ_fixture(runtime.clone(), "my-app-1").await;
        runtime.enable_app("my-app-1".into()).await.unwrap();

        let Provisioned(ProvisionedCell { cell_id, .. }) =
            app_info.cell_info.get("forum").unwrap().first().unwrap()
        else {
            panic!("App Info has no CellId")
        };

        // Subscribe before triggering any commit so we don't miss the signal.
        let mut signals = runtime
            .subscribe_to_app_signals("my-app-1".into())
            .await
            .unwrap();

        // create_post commits an entry (and a link); the posts zome's
        // post_commit hook emits a Signal::App for each, which must reach the
        // subscriber.
        #[derive(serde::Serialize, Debug)]
        struct Post {
            title: String,
            content: String,
        }
        let signed = runtime
            .sign_zome_call(ZomeCallParams {
                provenance: cell_id.agent_pubkey().clone(),
                cell_id: cell_id.clone(),
                zome_name: "posts".into(),
                fn_name: "create_post".into(),
                cap_secret: None,
                payload: ExternIO::encode(Post {
                    title: "hello".into(),
                    content: "world".into(),
                })
                .unwrap(),
                nonce: Nonce256Bits::from([1; 32]),
                expires_at: Timestamp(Timestamp::now().as_micros() + 60_000_000),
            })
            .await
            .unwrap();
        let resp = runtime
            .handle_app_request("my-app-1".into(), AppRequest::CallZome(Box::new(signed)))
            .await
            .unwrap();
        assert!(matches!(resp, AppResponse::ZomeCalled(_)));

        let signal = tokio::time::timeout(Duration::from_secs(30), signals.recv())
            .await
            .expect("timed out waiting for a signal")
            .expect("signal channel closed");
        assert!(matches!(signal, Signal::App { .. }));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_ensure_app_websocket() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        // An app only gets one app ws
        let app_websocket = runtime
            .ensure_app_websocket("my-app-1".into())
            .await
            .unwrap();
        let app_websocket_2 = runtime
            .ensure_app_websocket("my-app-1".into())
            .await
            .unwrap();
        let app_websocket_3 = {
            let all_app_auths = runtime.app_auths.read().unwrap();
            all_app_auths.get("my-app-1").unwrap().clone()
        };
        assert_eq!(app_websocket.port, app_websocket_2.port);
        assert_eq!(
            app_websocket.authentication.token,
            app_websocket_2.authentication.token
        );
        assert_eq!(
            app_websocket.authentication.expires_at,
            app_websocket_2.authentication.expires_at
        );
        assert_eq!(app_websocket_3.port, app_websocket.port);
        assert_eq!(
            app_websocket_3.authentication.token,
            app_websocket.authentication.token
        );
        assert_eq!(
            app_websocket_3.authentication.expires_at,
            app_websocket.authentication.expires_at
        );

        // Different apps get different ports and tokens
        let app_websocket_4 = runtime
            .ensure_app_websocket("my-app-2".into())
            .await
            .unwrap();
        assert_ne!(app_websocket_4.port, app_websocket.port);
        assert_ne!(
            app_websocket_4.authentication.token,
            app_websocket.authentication.token
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_api_err_bad_response() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let res = runtime.enable_app("non-existant-app-1".into()).await;
        assert!(res.is_err());
        assert!(matches!(res, Err(RuntimeError::AdminApiBadResponse { .. })))
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_setup_app_installs_when_app_id_different() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                    agent_key: None,
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    restore_from_dht: false,
                },
                false,
            )
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                    agent_key: None,
                    installed_app_id: Some("my-app-2".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    restore_from_dht: false,
                },
                false,
            )
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 2);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_setup_app_does_not_enable_after_install() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                    agent_key: None,
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    restore_from_dht: false,
                },
                false,
            )
            .await;
        assert!(res.is_ok());
        let apps = runtime.list_apps().await.unwrap();
        assert!(matches!(
            apps.first().unwrap().status,
            AppStatus::Disabled { .. }
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_setup_app_does_enable_after_install() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                network: RuntimeNetworkConfig::default(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                    agent_key: None,
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    restore_from_dht: false,
                },
                true,
            )
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert!(matches!(apps.first().unwrap().status, AppStatus::Enabled));
    }
}
