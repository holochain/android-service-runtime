use crate::error::RuntimeResultFfi;
use android_logger::Config;
use holochain_conductor_runtime::{move_to_locked_mem, ClientId, Runtime, RuntimeConfig};
use holochain_conductor_runtime_types_ffi::*;
use holochain_types::prelude::InstallAppPayload;
use log::{debug, LevelFilter};
use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime as TokioRuntime};
use url2::Url2;

/// Global multi threaded tokio runtime
pub static RT: LazyLock<TokioRuntime> =
    LazyLock::new(|| Builder::new_multi_thread().enable_all().build().unwrap());

/// Slim wrapper around HolochainRuntime, with types compatible with Uniffi-generated FFI bindings.
#[derive(uniffi::Object, Clone)]
pub struct RuntimeFfi(Runtime);

#[uniffi::export(async_runtime = "tokio")]
impl RuntimeFfi {
    #[uniffi::constructor]
    pub async fn start(
        passphrase: Vec<u8>,
        runtime_config: RuntimeConfigFfi,
    ) -> RuntimeResultFfi<Self> {
        android_logger::init_once(Config::default().with_max_level(LevelFilter::Warn));
        debug!("RuntimeFfi::new");

        let passphrase_locked =
            move_to_locked_mem(passphrase).expect("Failed to move password to locked memory");
        let runtime = Runtime::new(
            passphrase_locked,
            RuntimeConfig {
                data_root_path: runtime_config.data_root_path.into(),
                bootstrap_url: Url2::try_parse(runtime_config.bootstrap_url)?,
                signal_url: Url2::try_parse(runtime_config.signal_url)?,
            },
        )
        .await?;

        Ok(Self(runtime))
    }

    /// Shutdown the holochain conductor
    pub async fn stop(&self) -> RuntimeResultFfi<()> {
        debug!("RuntimeFfi::stop");
        Ok(self.0.stop().await?)
    }

    /// List apps installed on the conductor
    pub async fn list_apps(&self) -> RuntimeResultFfi<Vec<AppInfoFfi>> {
        debug!("RuntimeFfi::list_apps");

        Ok(self
            .0
            .list_apps()
            .await?
            .into_iter()
            .map(|a| a.into())
            .collect())
    }

    /// Is an app with the given installed_app_id installed on the conductor
    pub async fn is_app_installed(&self, installed_app_id: String) -> RuntimeResultFfi<bool> {
        debug!("RuntimeFfi::is_app_installed");
        Ok(self.0.is_app_installed(installed_app_id).await?)
    }

    /// Install an app
    pub async fn install_app(&self, payload: InstallAppPayloadFfi) -> RuntimeResultFfi<AppInfoFfi> {
        debug!("RuntimeFfi::install_app");
        let payload: InstallAppPayload = payload.try_into()?;

        #[cfg(not(test))]
        let res = RT.block_on(async { self.0.install_app(payload).await })?;

        #[cfg(test)]
        let res = self.0.install_app(payload).await?;

        Ok(res.into())
    }

    /// Uninstall an app
    pub async fn uninstall_app(&self, installed_app_id: String) -> RuntimeResultFfi<()> {
        debug!("RuntimeFfi::uninstall_app");
        Ok(self.0.uninstall_app(installed_app_id).await?)
    }

    /// Enable an installed app
    pub async fn enable_app(&self, installed_app_id: String) -> RuntimeResultFfi<AppInfoFfi> {
        debug!("RuntimeFfi::enable_app");
        Ok(self.0.enable_app(installed_app_id).await?.into())
    }

    /// Disable an installed app
    pub async fn disable_app(&self, installed_app_id: String) -> RuntimeResultFfi<()> {
        debug!("RuntimeFfi::disable_app");
        Ok(self.0.disable_app(installed_app_id).await?)
    }

    /// Get or create an app websocket with an authentication for the given app id
    pub async fn ensure_app_websocket(
        &self,
        installed_app_id: String,
    ) -> RuntimeResultFfi<AppAuthFfi> {
        debug!("RuntimeFfi::ensure_app_websocket");
        let app_auth = self.0.ensure_app_websocket(installed_app_id).await?;

        Ok(AppAuthFfi {
            authentication: app_auth.authentication.into(),
            port: app_auth.port,
        })
    }

    /// Setup app
    pub async fn setup_app(
        &self,
        payload: InstallAppPayloadFfi,
        enable_after_install: bool,
    ) -> RuntimeResultFfi<AppAuthFfi> {
        debug!("RuntimeFfi::setup_app");
        let payload: InstallAppPayload = payload.try_into()?;

        #[cfg(not(test))]
        let app_auth =
            RT.block_on(async { self.0.setup_app(payload, enable_after_install).await })?;

        #[cfg(test)]
        let app_auth = self
            .0
            .setup_app(payload.into(), enable_after_install)
            .await?;

        Ok(AppAuthFfi {
            authentication: app_auth.authentication.into(),
            port: app_auth.port,
        })
    }

    /// Sign a zome call
    pub async fn sign_zome_call(
        &self,
        zome_call_unsigned: ZomeCallUnsignedFfi,
    ) -> RuntimeResultFfi<ZomeCallFfi> {
        debug!("RuntimeFfi::sign_zome_call");
        Ok(self
            .0
            .sign_zome_call(zome_call_unsigned.into())
            .await?
            .into())
    }

    /// Authorize a client to call the given app id
    pub fn authorize_app_client(
        &self,
        client_id: String,
        installed_app_id: String,
    ) -> RuntimeResultFfi<()> {
        Ok(self
            .0
            .authorize_app_client(ClientId(client_id), installed_app_id)?)
    }

    /// Is this client authorized to call the given app id?
    pub fn is_app_client_authorized(
        &self,
        client_id: String,
        installed_app_id: String,
    ) -> RuntimeResultFfi<bool> {
        Ok(self
            .0
            .is_app_client_authorized(ClientId(client_id), installed_app_id)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::RuntimeErrorFfi;
    use std::collections::HashMap;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;
    use tempfile::TempDir;
    use uuid::Uuid;

    const HAPP_FIXTURE: &[u8] = include_bytes!("../fixtures/forum.happ");

    async fn install_happ_fixture(runtime: RuntimeFfi, app_id: &str) -> AppInfoFfi {
        runtime
            .install_app(InstallAppPayloadFfi {
                source: HAPP_FIXTURE.to_vec(),
                installed_app_id: Some(app_id.into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
            })
            .await
            .unwrap()
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_new_runtime() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();

        let bootstrap_url = "https://bootstrap.holo.host".to_string();
        let signal_url = "wss://sbd.holo.host".to_string();

        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: bootstrap_url.clone(),
                signal_url: signal_url.clone(),
            },
        )
        .await
        .unwrap();

        let res = runtime.list_apps().await;
        assert!(res.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_stop() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
            },
        )
        .await
        .unwrap();

        runtime.stop().await.unwrap();

        let res = runtime.list_apps().await;
        assert!(res.is_err());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_install_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .install_app(InstallAppPayloadFfi {
                source: HAPP_FIXTURE.to_vec(),
                installed_app_id: Some("my-app-1".into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
            })
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_uninstall_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
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
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
            },
        )
        .await
        .unwrap();
        install_happ_fixture(runtime.clone(), "my-app-1").await;

        let res = runtime.enable_app("my-app-1".into()).await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps.first().unwrap().status, AppInfoStatusFfi::Running);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_disable_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
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
            AppInfoStatusFfi::Disabled {
                reason: DisabledAppReasonFfi::User
            }
        ));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_apps() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
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
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
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
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
            },
        )
        .await
        .unwrap();

        let app_info = install_happ_fixture(runtime.clone(), "my-app-1").await;
        let CellInfoFfi::Provisioned(ProvisionedCellFfi { cell_id, .. }) =
            app_info.cell_info.get("forum").unwrap().first().unwrap()
        else {
            panic!("App Info has no CellId")
        };

        let res = runtime
            .sign_zome_call(ZomeCallUnsignedFfi {
                provenance: cell_id.agent_pub_key.clone(),
                cell_id: cell_id.clone(),
                zome_name: "forum".into(),
                fn_name: "get_all_posts".into(),
                cap_secret: None,
                payload: vec![],
                nonce: [0; 32].to_vec(),
                expires_at: i64::try_from(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_micros()
                        + 100000,
                )
                .unwrap(),
            })
            .await;
        assert!(res.is_ok())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_ensure_app_websocket() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
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
        assert_eq!(app_websocket.port, app_websocket_2.port);
        assert_eq!(
            app_websocket.authentication.token,
            app_websocket_2.authentication.token
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
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
            },
        )
        .await
        .unwrap();

        let res = runtime.enable_app("non-existant-app-1".into()).await;
        assert!(res.is_err());
        assert!(matches!(res, Err(RuntimeErrorFfi::Runtime { .. })))
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_setup_app() {
        let tmp_dir = TempDir::new().unwrap();
        let tmp_dir_path = tmp_dir.path().as_os_str().to_str().unwrap().to_string();
        let runtime = RuntimeFfi::start(
            vec![0, 0, 0, 0],
            RuntimeConfigFfi {
                data_root_path: tmp_dir_path,
                bootstrap_url: "https://bootstrap.holo.host".into(),
                signal_url: "wss://sbd.holo.host".into(),
            },
        )
        .await
        .unwrap();

        let res = runtime
            .setup_app(
                InstallAppPayloadFfi {
                    source: HAPP_FIXTURE.to_vec(),
                    installed_app_id: Some("my-app-1".into()),
                    network_seed: Some(Uuid::new_v4().to_string()),
                    roles_settings: Some(HashMap::new()),
                },
                true,
            )
            .await;
        assert!(res.is_ok());

        let apps = runtime.list_apps().await.unwrap();
        assert_eq!(apps.len(), 1);
    }
}
