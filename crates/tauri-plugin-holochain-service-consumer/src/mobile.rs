use crate::types::*;
use holochain_conductor_runtime_types_ffi::AppAuthFfi;
use serde::de::DeserializeOwned;
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
    /// Runs the entire process to setup a holochain app
    ///
    /// 1. Connect to holochain service
    /// 2. Check if app is already installed
    /// 3. If not installed, then install then app
    /// 4. Optionally, enable the app
    /// 5. Ensure there is an app websocket and return its authentication config
    ///
    /// This can be safely called whether or not the app is installed.
    pub fn setup_app(&self, config: SetupAppConfig) -> crate::Result<AppAuthFfi> {
        Ok(self.0.run_mobile_plugin::<AppAuthFfi>("setupApp", config)?)
    }

    /// Build a window that opens the main UI for your Tauri app.
    /// This is equivalent to creating a window with `WebviewUrl::App(PathBuf::from("index.html"))`.
    ///
    /// * `config`: The App Websocket config to inject into the webview
    pub fn main_window_builder(
        &self,
        app_id: String,
        auth: Option<AppAuthFfi>,
    ) -> tauri::Result<WebviewWindowBuilder<R, AppHandle<R>>> {
        let label = "main";
        let mut window_builder =
            WebviewWindowBuilder::new(self.0.app(), label, WebviewUrl::App("".into()))
                .initialization_script(include_str!("../dist-js/holochain-env/index.min.js"));

        if let Some(auth) = auth {
            window_builder = window_builder.initialization_script(
                format!(
                    r#"injectHolochainClientEnv("{}", {}, {:?});"#,
                    app_id, auth.port, auth.authentication.token,
                )
                .as_str(),
            );
        }

        // Attach necessary capabilities to window
        let mut capability_builder =
            CapabilityBuilder::new("default").permission("holochain-service-consumer:default");
        capability_builder = capability_builder.window(label);
        self.0.app().add_capability(capability_builder)?;

        Ok(window_builder)
    }

    /// Setup a holochain app, then create the main window builder which will inject the holochain env JS on initialization.
    /// 
    /// Note you must call `.build()` on the returned WebviewWindowBuilder to actually build the window.
    pub fn setup_app_main_window(
        &self,
        config: SetupAppConfig,
    ) -> tauri::Result<WebviewWindowBuilder<R, AppHandle<R>>> {
        let app_auth = self.setup_app(config.clone()).ok();
        let window_builder = self.main_window_builder(config.app_id, app_auth.clone())?;
        
        Ok(window_builder)
    }
}
