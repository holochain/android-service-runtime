use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::ops::Deref;
use tauri::{
    ipc::CapabilityBuilder,
    plugin::{PluginApi, PluginHandle},
    AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder,
};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.holochain_service_consumer";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_holochain_service_consumer);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<HolochainServiceConsumer<R>> {
    #[cfg(target_os = "android")]
    let handle =
        api.register_android_plugin(PLUGIN_IDENTIFIER, "HolochainServiceConsumerPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_holochain_service - consumer)?;
    Ok(HolochainServiceConsumer(handle))
}

/// Access to the holochain-service-consumer APIs.
pub struct HolochainServiceConsumer<R: Runtime>(pub PluginHandle<R>);

impl<R: Runtime> HolochainServiceConsumer<R> {
    /// Build a window that opens the main UI for your Tauri app.
    /// This is equivalent to creating a window with `WebviewUrl::App(PathBuf::from("index.html"))`.
    ///
    /// * `app_id` - The `app_id` for the app.
    ///   If an app with this `app_id` is not installed in the Holochain conductor running in the Android Service Runtime,
    ///   then this happ bundle will be installed with the provided `app_id` and `network_seed`.
    ///   Afterwards, the window will be setup so a holochain client can connect to that app.
    /// * `happ_bundle_bytes` - The raw bytes of the `.happ` file.
    /// * `network_seed` - The network seed to include in the `InstallAppPayload`.
    /// * `enable_app` - If `true`, enable_app will be called for the app, after it has been installed successfully.
    pub fn main_window_builder(
        &self,
        app_id: String,
        happ_bundle_bytes: Bytes,
        network_seed: String,
        enable_app: bool
    ) -> tauri::Result<WebviewWindowBuilder<R, AppHandle<R>>> {
        let label = "main";
        let window_builder =
            WebviewWindowBuilder::new(self.0.app(), label, WebviewUrl::App("".into()))
                .initialization_script(include_str!("../dist-js/holochain-env/index.min.js"))
                .initialization_script(
                    format!(
                        r#"setupApp("{}", {:?}, "{}", {});"#,
                        app_id,
                        happ_bundle_bytes.deref(),
                        network_seed,
                        enable_app,
                    )
                    .as_str(),
                );

        // Attach necessary capabilities to window
        let mut capability_builder =
            CapabilityBuilder::new("default").permission("holochain-service-consumer:default");
        capability_builder = capability_builder.window(label);
        self.0.app().add_capability(capability_builder)?;

        Ok(window_builder)
    }
}
