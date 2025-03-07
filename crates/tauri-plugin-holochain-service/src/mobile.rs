use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.holochain_service";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain_service);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<HolochainService<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainServicePlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_holochain_service)?;
    Ok(HolochainService(handle))
}

/// Access to the holochain-service APIs.
pub struct HolochainService<R: Runtime>(pub PluginHandle<R>);
