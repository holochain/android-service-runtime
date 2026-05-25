//! A Tauri plugin that runs a Holochain conductor **in-process** — no UniFFI,
//! no Kotlin, no separate Android service, no cross-process AIDL.
//!
//! It is built directly on [`holochain_conductor_runtime::Runtime`], the same
//! pure-Rust conductor wrapper used by the FFI-based client/service plugins, and
//! exposes it to a Tauri app via the [`HolochainExt`] trait. A webview opened
//! with [`HolochainPlugin::main_window_builder`] is wired to the in-process
//! conductor's app websocket so `@holochain/client` in the UI connects directly.
//!
//! This is the single-binary alternative to the separated
//! `tauri-plugin-holochain-service` (server) + `tauri-plugin-holochain-service-client`
//! (client) pair; those remain supported for the cross-app Android model.

mod commands;
mod error;

pub use error::{Error, Result};

// Re-export the native config type consumers build, and the runtime itself.
pub use holochain::conductor::config::NetworkConfig;
pub use holochain_conductor_runtime::Runtime;
pub use lair_keystore_api::types::SharedLockedArray;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use sodoken::LockedArray;
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Emitter, Manager, Runtime as TauriRuntime, WebviewUrl, WebviewWindowBuilder,
};

/// Emitted on the app handle once the conductor is up and [`HolochainExt::holochain`]
/// is ready to use.
pub const EVENT_READY: &str = "holochain://ready";
/// Emitted on the app handle if the conductor fails to start. The payload is the
/// error string.
pub const EVENT_SETUP_FAILED: &str = "holochain://setup-failed";

/// Configuration for the in-process Holochain conductor.
pub struct HolochainPluginConfig {
    /// Directory where conductor + keystore data is stored.
    pub data_dir: PathBuf,
    /// Native holochain network config (bootstrap / signal / relay / ...).
    pub network: NetworkConfig,
}

impl HolochainPluginConfig {
    pub fn new(data_dir: PathBuf, network: NetworkConfig) -> Self {
        Self { data_dir, network }
    }
}

/// Wrap a passphrase byte vector in the shared, memory-locked form lair expects.
pub fn vec_to_locked(pass: Vec<u8>) -> SharedLockedArray {
    Arc::new(Mutex::new(LockedArray::from(pass)))
}

/// Options for [`HolochainPlugin::main_window_builder`].
#[derive(Default)]
pub struct WindowOptions {
    /// Webview URL to load. Defaults to the app's `index.html`.
    pub url: Option<WebviewUrl>,
    /// Window title (desktop).
    pub title: Option<String>,
}

/// Access to the running in-process Holochain conductor from the Tauri app.
pub struct HolochainPlugin<R: TauriRuntime> {
    runtime: Runtime,
    app_handle: AppHandle<R>,
}

impl<R: TauriRuntime> HolochainPlugin<R> {
    /// The underlying conductor runtime. The full lifecycle API
    /// (`install_app`, `enable_app`, `setup_app`, `ensure_app_websocket`,
    /// `sign_zome_call`, `import_key_seed`, ...) lives here — this plugin is a
    /// thin Tauri adapter, not a re-implementation.
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// Ensure an app websocket exists for `app_id`, then build a webview window
    /// wired to it: it injects the standard `__HC_LAUNCHER_ENV__`
    /// (`INSTALLED_APP_ID` / `APP_INTERFACE_PORT` / `APP_INTERFACE_TOKEN`) plus a
    /// zome-call signer, so `@holochain/client` in the webview connects to the
    /// in-process conductor and can make authenticated zome calls.
    ///
    /// Call `.build()` on the returned builder to actually open the window.
    pub async fn main_window_builder(
        &self,
        label: impl Into<String>,
        app_id: String,
        options: WindowOptions,
    ) -> Result<WebviewWindowBuilder<'_, R, AppHandle<R>>> {
        let app_auth = self.runtime.ensure_app_websocket(app_id.clone()).await?;

        let mut window_builder = WebviewWindowBuilder::new(
            &self.app_handle,
            label,
            options
                .url
                .unwrap_or_else(|| WebviewUrl::App("index.html".into())),
        )
        .initialization_script(include_str!("../dist-js/holochain-env/index.min.js"))
        .initialization_script(
            format!(
                r#"window.injectHolochainClientEnv("{}", {}, {:?});"#,
                app_id,
                app_auth.port,
                app_auth.authentication.token,
            )
            .as_str(),
        );

        if let Some(title) = options.title {
            window_builder = window_builder.title(title);
        }

        Ok(window_builder)
    }
}

/// Extension trait giving Tauri's `App`/`AppHandle`/`Window` access to the
/// in-process Holochain conductor.
pub trait HolochainExt<R: TauriRuntime> {
    /// Access the running Holochain plugin.
    ///
    /// Returns [`Error::NotReady`] until the conductor has finished starting;
    /// listen for [`EVENT_READY`] before calling this.
    fn holochain(&self) -> Result<&HolochainPlugin<R>>;
}

impl<R: TauriRuntime, T: Manager<R>> HolochainExt<R> for T {
    fn holochain(&self) -> Result<&HolochainPlugin<R>> {
        self.try_state::<HolochainPlugin<R>>()
            .map(|state| state.inner())
            .ok_or(Error::NotReady)
    }
}

/// Initialize the plugin.
///
/// The conductor is booted asynchronously so the Tauri app can show a
/// splashscreen while it starts. [`EVENT_READY`] is emitted once
/// [`HolochainExt::holochain`] is usable; [`EVENT_SETUP_FAILED`] is emitted on
/// failure.
pub fn init<R: TauriRuntime>(
    passphrase: SharedLockedArray,
    config: HolochainPluginConfig,
) -> TauriPlugin<R> {
    Builder::new("holochain")
        .invoke_handler(tauri::generate_handler![commands::sign_zome_call])
        .setup(move |app, _api| {
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                match Runtime::new_with_network_config(passphrase, config.data_dir, config.network)
                    .await
                {
                    Ok(runtime) => {
                        app_handle.manage(HolochainPlugin {
                            runtime,
                            app_handle: app_handle.clone(),
                        });
                        if let Err(e) = app_handle.emit(EVENT_READY, ()) {
                            log::error!("Failed to emit {EVENT_READY}: {e:?}");
                        }
                    }
                    Err(e) => {
                        log::error!("Holochain conductor setup failed: {e:?}");
                        let _ = app_handle.emit(EVENT_SETUP_FAILED, e.to_string());
                    }
                }
            });
            Ok(())
        })
        .build()
}
