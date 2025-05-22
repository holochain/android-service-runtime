use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};
use holochain_conductor_runtime_types_ffi::RuntimeNetworkConfigFfi;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "org.holochain.androidserviceruntime.plugin.service";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain_service);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
    config: RuntimeNetworkConfigFfi,
) -> crate::Result<HolochainService<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainServicePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_holochain_service)?;

    // Set the default runtime config
    let holochain_service = HolochainService(handle);
    let _ = holochain_service.set_config(config)?;

    Ok(holochain_service)
}

/// Access to the holochain-service APIs.
pub struct HolochainService<R: Runtime>(pub PluginHandle<R>);

impl<R: Runtime> HolochainService<R> {
    pub fn set_config(&self, config: RuntimeNetworkConfigFfi) -> crate::Result<()> {
        Ok(self
            .0
            .run_mobile_plugin::<()>("setConfig", config)?)
    }
}