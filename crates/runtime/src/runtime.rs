use crate::{AuthorizedAppClientsManager, ClientId};
use crate::{AppAuth, RuntimeConfig, RuntimeError, RuntimeResult, DEVICE_SEED_LAIR_TAG};
use holochain::conductor::api::AppAuthenticationTokenIssued;
use holochain::conductor::api::IssueAppAuthenticationTokenPayload;
use holochain::{
    conductor::{
        api::{AdminInterfaceApi, AdminRequest, AdminResponse, AppInfo, ZomeCall},
        ConductorBuilder, ConductorHandle,
    },
    prelude::{InstallAppPayload, InstalledAppId, ZomeCallUnsigned},
};
use holochain_types::websocket::AllowedOrigins;
use kitsune_p2p_types::dependencies::lair_keystore_api::dependencies::sodoken::BufRead;
use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Map of app ids to their associated app websocket & authentication
pub type AppAuths = Arc<RwLock<HashMap<InstalledAppId, AppAuth>>>;

/// Slim wrapper around holochain Conductor with calls wrapping AdminInterfaceApi requests
#[derive(Clone)]
pub struct Runtime {
    conductor: ConductorHandle,
    app_auths: AppAuths,
    authorized_app_clients: Arc<AuthorizedAppClientsManager>
}

impl Runtime {
    /// Initialize and start a new Conductor
    pub async fn new(passphrase: BufRead, runtime_config: RuntimeConfig) -> RuntimeResult<Self> {
        let conductor = ConductorBuilder::default()
            .passphrase(Some(passphrase))
            .config(runtime_config.clone().into())
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
            authorized_app_clients: Arc::new(AuthorizedAppClientsManager::new(runtime_config.data_root_path)),
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
            AdminResponse::AppEnabled { app, errors } => {
                if errors.is_empty() {
                    Ok(app)
                } else {
                    Err(RuntimeError::AdminApiAppEnabled(errors))
                }
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

    pub async fn is_app_installed(&self, installed_app_id: InstalledAppId) -> RuntimeResult<bool> {
        Ok(self
            .list_apps()
            .await?
            .into_iter()
            .any(|app_info| app_info.installed_app_id == installed_app_id))
    }

    pub async fn sign_zome_call(
        &self,
        zome_call_unsigned: ZomeCallUnsigned,
    ) -> RuntimeResult<ZomeCall> {
        ZomeCall::try_from_unsigned_zome_call(self.conductor.keystore(), zome_call_unsigned)
            .await
            .map_err(RuntimeError::Lair)
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
        self.authorized_app_clients.authorize(client_uid, installed_app_id)
    }

    pub fn is_app_client_authorized(
        &self,
        client_uid: ClientId,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<bool> {
        self.authorized_app_clients.is_authorized(client_uid, installed_app_id)
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
    use super::*;
    use holochain::conductor::api::AppInfoStatus;
    use holochain::conductor::api::CellInfo::Provisioned;
    use holochain::conductor::api::ProvisionedCell;
    use holochain::conductor::config::KeystoreConfig;
    use holochain_types::prelude::AppBundleSource;
    use holochain_types::prelude::DisabledAppReason;
    use holochain_types::prelude::Nonce256Bits;
    use holochain_types::prelude::Timestamp;
    use kitsune_p2p_types::config::TransportConfig;
    use tempfile::TempDir;
    use url2::Url2;
    use uuid::Uuid;

    const HAPP_FIXTURE: &[u8] = include_bytes!("../fixtures/forum.happ");

    async fn install_happ_fixture(runtime: Runtime, app_id: &str) -> AppInfo {
        runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec()),
                agent_key: None,
                installed_app_id: Some(app_id.into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                allow_throwaway_random_agent_key: false,
            })
            .await
            .unwrap()
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_new_runtime() {
        let tmp_dir = TempDir::new().unwrap();
        let bootstrap_url = Url2::try_parse("https://bootstrap.holo.host").unwrap();
        let signal_url = Url2::try_parse("wss://sbd.holo.host").unwrap();

        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir.path().into(),
                bootstrap_url: bootstrap_url.clone(),
                signal_url: signal_url.clone(),
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
            runtime
                .conductor
                .config
                .network
                .bootstrap_service
                .clone()
                .unwrap(),
            bootstrap_url
        );
        assert_eq!(
            runtime
                .conductor
                .config
                .network
                .transport_pool
                .first()
                .unwrap()
                .clone(),
            TransportConfig::WebRTC {
                signal_url: signal_url.into(),
                webrtc_config: None
            }
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
    async fn test_install_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec()),
                agent_key: None,
                installed_app_id: Some("my-app-1".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                allow_throwaway_random_agent_key: false,
            })
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_uninstall_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;

        let res = runtime.enable_app("my-app-1".into()).await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps.first().unwrap().status, AppInfoStatus::Running);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_disable_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            AppInfoStatus::Disabled {
                reason: DisabledAppReason::User
            }
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_apps() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            .sign_zome_call(ZomeCallUnsigned {
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
    async fn test_ensure_app_websocket() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec()),
                    agent_key: None,
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    allow_throwaway_random_agent_key: false,
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
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec()),
                    agent_key: None,
                    installed_app_id: Some("my-app-2".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    allow_throwaway_random_agent_key: false,
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
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec()),
                    agent_key: None,
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    allow_throwaway_random_agent_key: false,
                },
                false,
            )
            .await;
        assert!(res.is_ok());
        let apps = runtime.list_apps().await.unwrap();
        assert!(matches!(
            apps.first().unwrap().status,
            AppInfoStatus::Disabled { .. }
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_setup_app_does_enable_after_install() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().to_path_buf();
        let runtime = Runtime::new(
            BufRead::from(vec![0, 0, 0, 0]),
            RuntimeConfig {
                data_root_path: tmp_dir_path,
                bootstrap_url: Url2::try_parse("https://bootstrap.holo.host").unwrap(),
                signal_url: Url2::try_parse("wss://sbd.holo.host").unwrap(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayload {
                    source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec()),
                    agent_key: None,
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                    ignore_genesis_failure: false,
                    allow_throwaway_random_agent_key: false,
                },
                true,
            )
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert!(matches!(
            apps.first().unwrap().status,
            AppInfoStatus::Running
        ));
    }
}
