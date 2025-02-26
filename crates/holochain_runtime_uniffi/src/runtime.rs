use holochain_runtime_types_uniffi::config::HolochainRuntimeFFIConfig;
use holochain_runtime_types_uniffi::error::HolochainRuntimeFFIError;
use holochain_runtime_types_uniffi::types::{AppInfoFFI, RoleSettingsFFI, ZomeCallFFI, ZomeCallUnsignedFFI, AppWebsocketAuthFFI};
use holochain_runtime::{HolochainRuntime, vec_to_locked};
use holochain_types::{prelude::{AgentPubKey, AppBundle}, websocket::AllowedOrigins, app::RoleSettings};
use log::LevelFilter;
use android_logger::Config;
use std::collections::HashMap;

/// Wrapper around HolochainRuntime with types that can be exposed to FFI.
#[derive(uniffi::Object)]
pub struct HolochainRuntimeFFI {
    runtime: HolochainRuntime,
}

#[uniffi::export(async_runtime = "tokio")]
impl HolochainRuntimeFFI {

    /// Start the holochain conductor
    #[uniffi::constructor]
    pub async fn launch(passphrase: Vec<u8>, config: HolochainRuntimeFFIConfig) -> Result<Self, HolochainRuntimeFFIError> {
        android_logger::init_once(
            Config::default().with_max_level(LevelFilter::Warn),
        );
        
        let passphrase_bufread = vec_to_locked(passphrase).map_err(|e| HolochainRuntimeFFIError::IOError(e.to_string()))?;

        let runtime = HolochainRuntime::launch(
                passphrase_bufread,
                config.try_into()?
            )
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        
        Ok(HolochainRuntimeFFI {
            runtime
        })
    }

    /// Shutdown the holochain conductor
    pub async fn shutdown(&self) -> Result<(), HolochainRuntimeFFIError> {
        self.runtime.shutdown().await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Get an admin port on the conductor
    pub fn get_admin_port(&self) -> u16 {
        self.runtime.admin_port
    }

    /// List apps installed on the conductor
    pub async fn list_installed_apps(&self) -> Result<Vec<AppInfoFFI>, HolochainRuntimeFFIError> {
        let apps = self.runtime.admin_websocket().await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?
            .list_apps(None).await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(format!("{:?}", e)))?
            .into_iter()
            .map(|a| a.into())
            .collect();
        
        Ok(apps)
    }

    /// Is an app with the given installed_app_id installed on the conductor
    pub async fn is_app_installed(&self, installed_app_id: String) -> Result<bool, HolochainRuntimeFFIError> {
        self.runtime.is_app_installed(installed_app_id).await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(format!("{:?}", e)))
    }

    /// Install an app
    pub async fn install_app(
        &self,
        app_id: String,
        app_bundle_bytes: Vec<u8>,
        roles_settings: Option<HashMap<String, RoleSettingsFFI>>,
        agent: Option<Vec<u8>>,
        network_seed: Option<String>
    ) -> Result<(), HolochainRuntimeFFIError> {
        let agent = agent.and_then(|a| 
                Some(AgentPubKey::from_raw_39(a))
            ).transpose()
            .map_err(|e| HolochainRuntimeFFIError::Infallible(e.to_string()))?;
        let app_bundle = AppBundle::decode(app_bundle_bytes.as_slice())
            .map_err(|e| HolochainRuntimeFFIError::Infallible(e.to_string()))?;
        let roles_settings: Option<HashMap<String, RoleSettings>> = roles_settings.map(|r| {
            r.into_iter().map(|(key, val)| (key, val.into())).collect()
        });
        
        self.runtime.install_app(app_id, app_bundle, roles_settings, agent, network_seed)
           .await
           .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Uninstall an app
    pub async fn uninstall_app(
        &self,
        app_id: String,
    ) -> Result<(), HolochainRuntimeFFIError> {        
        self.runtime.uninstall_app(app_id)
           .await
           .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Enable an installed app
    pub async fn enable_app(
        &self,
        app_id: String,
    ) -> Result<(), HolochainRuntimeFFIError> {        
        self.runtime.enable_app(app_id)
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }

    /// Disable an installed app
    pub async fn disable_app(
        &self,
        app_id: String,
    ) -> Result<(), HolochainRuntimeFFIError> {        
        self.runtime.disable_app(app_id)
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(())
    }
    
    /// Get or create an app websocket with an authentication for the given app id
    pub async fn app_websocket_auth(&self, app_id: String) -> Result<AppWebsocketAuthFFI, HolochainRuntimeFFIError> {
        let app_websocket_auth = self.runtime.get_app_websocket_auth(&app_id, AllowedOrigins::Any)
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(app_websocket_auth.into())
    }

    /// Sign a zome call
    pub async fn sign_zome_call(&self, zome_call_unsigned: ZomeCallUnsignedFFI) -> Result<ZomeCallFFI, HolochainRuntimeFFIError> {
        let zome_call = self.runtime.sign_zome_call(zome_call_unsigned.into())
            .await
            .map_err(|e| HolochainRuntimeFFIError::HolochainError(e.to_string()))?;
        Ok(zome_call.into())
    }
}
