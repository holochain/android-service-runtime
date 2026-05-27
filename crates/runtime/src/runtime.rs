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
    prelude::{AgentPubKey, CellId, InstallAppPayload, InstalledAppId, ZomeCallParams},
};
use holochain_types::signal::Signal;
use holochain_types::websocket::AllowedOrigins;
use kitsune2_api::ApiTransportStats;
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
        Self::new_with_conductor_config(passphrase, data_root_path, runtime_config.into()).await
    }

    /// Initialize and start a new Conductor from a native holochain [`NetworkConfig`].
    ///
    /// This is the entry point for the in-process Tauri plugin, which works with
    /// holochain's native config types directly rather than the FFI wrappers.
    pub async fn new_with_network_config(
        passphrase: SharedLockedArray,
        data_root_path: PathBuf,
        network: NetworkConfig,
    ) -> RuntimeResult<Self> {
        let config = ConductorConfig {
            data_root_path: Some(data_root_path.clone().into()),
            keystore: KeystoreConfig::LairServerInProc { lair_root: None },
            network,
            ..Default::default()
        };
        Self::new_with_conductor_config(passphrase, data_root_path, config).await
    }

    /// Shared constructor: build the conductor from a fully-formed [`ConductorConfig`],
    /// ensure the device seed exists, and set up the runtime's bookkeeping.
    async fn new_with_conductor_config(
        passphrase: SharedLockedArray,
        data_root_path: PathBuf,
        config: ConductorConfig,
    ) -> RuntimeResult<Self> {
        let res = rustls::crypto::aws_lc_rs::default_provider().install_default();
        if res.is_err() {
            error!("Failed to set default crypto provider for tls: {:?}", res);
        }

        let conductor = ConductorBuilder::default()
            .passphrase(Some(passphrase))
            .config(config)
            .build()
            .await?;

        let has_device_seed = conductor
            .keystore()
            .lair_client()
            .get_entry(DEVICE_SEED_LAIR_TAG.into())
            .await
            .is_ok();
        if !has_device_seed {
            conductor
                .keystore()
                .lair_client()
                .new_seed(DEVICE_SEED_LAIR_TAG.into(), None, true)
                .await
                .map_err(RuntimeError::Lair)?;
        }

        Ok(Self {
            conductor,
            app_auths: Arc::new(RwLock::new(HashMap::new())),
            authorized_app_clients: Arc::new(AuthorizedAppClientsManager::new(
                data_root_path,
            )?),
        })
    }

    /// Stop the Conductor
    ///
    /// This is *NOT* fully implemented by holochain: kitsune tasks will continue to run.
    pub async fn stop(&self) -> RuntimeResult<()> {
        self.conductor
            .shutdown()
            .await
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?;

        Ok(())
    }

    pub async fn install_app(&self, payload: InstallAppPayload) -> RuntimeResult<AppInfo> {
        let response = self
            .req_admin_api(AdminRequest::InstallApp(Box::new(payload)))
            .await?;
        match response {
            AdminResponse::AppInstalled(app_info) => Ok(app_info),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
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
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn enable_app(&self, installed_app_id: InstalledAppId) -> RuntimeResult<AppInfo> {
        let response = self
            .req_admin_api(AdminRequest::EnableApp { installed_app_id })
            .await?;
        match response {
            AdminResponse::AppEnabled(app) => {
                Ok(app)
            }
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn disable_app(&self, installed_app_id: InstalledAppId) -> RuntimeResult<()> {
        let response = self
            .req_admin_api(AdminRequest::DisableApp { installed_app_id })
            .await?;
        match response {
            AdminResponse::AppDisabled => Ok(()),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
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
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    /// Dump the conductor's networking stats.
    ///
    /// Returns the typed [`ApiTransportStats`] (not a stringified dump) so callers
    /// can read fields like `transport_stats.backend` directly. This is the
    /// in-process equivalent of an admin `DumpNetworkStats` request.
    pub async fn dump_network_stats(&self) -> RuntimeResult<ApiTransportStats> {
        let response = self.req_admin_api(AdminRequest::DumpNetworkStats).await?;
        match response {
            AdminResponse::NetworkStatsDumped(stats) => Ok(stats),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn is_app_installed(&self, installed_app_id: InstalledAppId) -> RuntimeResult<bool> {
        Ok(self
            .list_apps()
            .await?
            .into_iter()
            .any(|app_info| app_info.installed_app_id == installed_app_id))
    }

    pub async fn import_key_seed(&self, seed: [u8; 32]) -> RuntimeResult<AgentPubKey> {
        let client = self.conductor.keystore().lair_client();

        // Generate a temporary local x25519 keypair for the sender side.
        // This keypair never enters Lair — it is only used to box-encrypt the seed.
        let mut sender_pk = [0u8; sodoken::crypto_box::XSALSA_PUBLICKEYBYTES];
        let mut sender_sk = sodoken::SizedLockedArray::new().map_err(|e| RuntimeError::Lair(e.into()))?;
        sodoken::crypto_box::xsalsa_keypair(&mut sender_pk, &mut sender_sk.lock())
            .map_err(|e| RuntimeError::Lair(e.into()))?;

        // Generate a second temporary local x25519 keypair for the recipient side.
        // Lair's `import_seed` decrypts with the private key of the recipient entry,
        // so we first need to create a Lair seed entry to act as the recipient.
        let recipient_info = client
            .new_seed(uuid::Uuid::new_v4().to_string().into(), None, false)
            .await
            .map_err(RuntimeError::Lair)?;
        let recipient_pk = recipient_info.x25519_pub_key;

        // Box-encrypt the seed bytes for the recipient.
        let mut nonce = [0u8; sodoken::crypto_box::XSALSA_NONCEBYTES];
        sodoken::random::randombytes_buf(&mut nonce).map_err(|e| RuntimeError::Lair(e.into()))?;
        let mut cipher = vec![0u8; 32 + sodoken::crypto_box::XSALSA_MACBYTES];
        sodoken::crypto_box::xsalsa_easy(&mut cipher, &seed, &nonce, &recipient_pk, &sender_sk.lock())
            .map_err(|e| RuntimeError::Lair(e.into()))?;

        // Import the encrypted seed into Lair under a fresh tag.
        // Lair uses the recipient entry's private key to decrypt and store the seed.
        let seed_info = client
            .import_seed(
                sender_pk.into(),
                recipient_pk,
                None,
                nonce,
                cipher.into(),
                uuid::Uuid::new_v4().to_string().into(),
                false,
            )
            .await
            .map_err(RuntimeError::Lair)?;

        Ok(AgentPubKey::from_raw_32(seed_info.ed25519_pub_key.0.to_vec()))
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
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
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
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::RuntimeNetworkConfig;

    use super::*;
    use holochain::conductor::api::CellInfo::Provisioned;
    use holochain::conductor::api::ProvisionedCell;
    use holochain::conductor::config::KeystoreConfig;
    use holochain_types::prelude::AppBundleSource;
    use holochain_types::prelude::DisabledAppReason;
    use holochain_types::prelude::ExternIO;
    use holochain_types::prelude::Link;
    use holochain_types::prelude::Nonce256Bits;
    use holochain_types::prelude::Timestamp;
    use holochain_types::prelude::AppStatus;

    use serde_json::json;
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
            })
            .await
            .unwrap()
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_new_runtime() {
        let tmp_dir = TempDir::new().unwrap();
        let bootstrap_url = Url2::try_parse("https://bootstrap.com").unwrap();
        let signal_url = Url2::try_parse("wss://signal.com").unwrap();
        let relay_url = Url2::try_parse("https://relay.com").unwrap();
        let stun_url = Url2::try_parse("stun:stun.com:1234").unwrap();
        let ice_urls = vec![stun_url.clone()];

        let runtime = Runtime::new(
            Arc::new(Mutex::new(LockedArray::from(vec![0, 0, 0, 0]))),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                network: RuntimeNetworkConfig {
                    bootstrap_url: bootstrap_url.clone(),
                    signal_url: signal_url.clone(),
                    relay_url: relay_url.clone(),
                    ice_urls: ice_urls.clone(),
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
            runtime.conductor.config.network.signal_url.clone(),
            signal_url
        );
        assert_eq!(
            runtime.conductor.config.network.relay_url.clone(),
            relay_url
        );
        assert_eq!(
            runtime
                .conductor
                .config
                .network
                .webrtc_config
                .clone()
                .unwrap(),
            json!({
                "iceServers": [
                    { "urls": [stun_url.to_string()] },
                ]
            })
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
            })
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
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
            })
            .await
            .unwrap();

        // The installed app uses the imported agent key rather than a fresh one.
        assert_eq!(app_info.agent_pub_key, agent_key);
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
            AppStatus::Disabled (
                DisabledAppReason::User
            )
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
                },
                true,
            )
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert!(matches!(
            apps.first().unwrap().status,
            AppStatus::Enabled
        ));
    }
}
