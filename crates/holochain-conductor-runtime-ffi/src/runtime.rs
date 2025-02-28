use crate::error::RuntimeResultFfi;
use crate::types::{AppInfoFfi, InstallAppPayloadFfi, ZomeCallFfi, ZomeCallUnsignedFfi};
use crate::{config::RuntimeConfigFfi, types::AppWebsocketFfi};
use android_logger::Config;
use holochain_conductor_runtime::{move_to_locked_mem, Runtime};
use log::LevelFilter;

/// Slim wrapper around HolochainRuntime, with types compatible with Uniffi-generated FFI bindings.
#[derive(uniffi::Object)]
pub struct RuntimeFfi(Runtime);

#[uniffi::export(async_runtime = "tokio")]
impl RuntimeFfi {
    #[uniffi::constructor]
    pub async fn new(
        passphrase: Option<Vec<u8>>,
        runtime_config: RuntimeConfigFfi,
    ) -> RuntimeResultFfi<Self> {
        let passphrase_locked = passphrase.map(move_to_locked_mem).transpose()?;
        android_logger::init_once(Config::default().with_max_level(LevelFilter::Warn));
        let runtime = Runtime::new(
            passphrase_locked,
            runtime_config.try_into()?,
        )
        .await?;

        Ok(Self(runtime))
    }

    /// Shutdown the holochain conductor
    pub async fn stop(&self) -> RuntimeResultFfi<()> {
        Ok(self.0.stop().await?)
    }

    /// Get an admin port on the conductor
    /// TODO: DELETE THIS
    pub fn get_admin_port(&self) -> u16 {
        12345
    }

    /// List apps installed on the conductor
    pub async fn list_apps(&self) -> RuntimeResultFfi<Vec<AppInfoFfi>> {
        Ok(self.0.list_apps().await?.into_iter().map(|a| a.into()).collect())
    }

    /// Is an app with the given installed_app_id installed on the conductor
    pub async fn is_app_installed(&self, installed_app_id: String) -> RuntimeResultFfi<bool> {
        Ok(self.0.is_app_installed(installed_app_id).await?)
    }

    /// Install an app
    pub async fn install_app(&self, payload: InstallAppPayloadFfi) -> RuntimeResultFfi<AppInfoFfi> {
        Ok(self.0.install_app(payload.try_into()?).await?.into())
    }

    /// Uninstall an app
    pub async fn uninstall_app(&self, installed_app_id: String) -> RuntimeResultFfi<()> {
        Ok(self.0.uninstall_app(installed_app_id).await?)
    }

    /// Enable an installed app
    pub async fn enable_app(&self, installed_app_id: String) -> RuntimeResultFfi<AppInfoFfi> {
        Ok(self.0.enable_app(installed_app_id).await?.into())
    }

    /// Disable an installed app
    pub async fn disable_app(&self, installed_app_id: String) -> RuntimeResultFfi<()> {
        Ok(self.0.disable_app(installed_app_id).await?)
    }

    /// Get or create an app websocket with an authentication for the given app id
    pub async fn ensure_app_websocket(
        &self,
        installed_app_id: String,
    ) -> RuntimeResultFfi<AppWebsocketFfi> {
        Ok(self
            .0
            .ensure_app_websocket(installed_app_id)
            .await?
            .into())
    }

    /// Sign a zome call
    pub async fn sign_zome_call(
        &self,
        zome_call_unsigned: ZomeCallUnsignedFfi,
    ) -> RuntimeResultFfi<ZomeCallFfi> {
        Ok(self
            .0
            .sign_zome_call(zome_call_unsigned.into())
            .await?
            .into())
    }
}
