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

use holochain::conductor::api::AppRequest;
use holochain::prelude::{decode, encode, InstalledAppId};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

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
/// Emitted to a direct-mode webview window with the msgpack-encoded conductor
/// [`holochain_types::signal::Signal`] for the app the window is bound to. The
/// injected env bridges this to `@holochain/client`'s Tauri transport.
pub const EVENT_SIGNAL: &str = "holochain://signal";

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
    /// Use the legacy app-websocket wiring (attach an app interface and inject
    /// `__HC_LAUNCHER_ENV__`) instead of direct Tauri IPC. Defaults to `false`
    /// (direct), which needs no loopback websocket.
    pub use_app_websocket: bool,
}

/// Access to the running in-process Holochain conductor from the Tauri app.
pub struct HolochainPlugin<R: TauriRuntime> {
    runtime: Runtime,
    app_handle: AppHandle<R>,
    /// Maps a webview window label to the app it is bound to. Populated by
    /// [`HolochainPlugin::main_window_builder`] / [`HolochainPlugin::bind_window`].
    /// The `app_request` command uses this to scope each request to the app the
    /// calling window was opened for — replacing the per-app websocket token.
    window_apps: Arc<Mutex<HashMap<String, InstalledAppId>>>,
}

impl<R: TauriRuntime> HolochainPlugin<R> {
    /// The underlying conductor runtime. The full lifecycle API
    /// (`install_app`, `enable_app`, `setup_app`, `ensure_app_websocket`,
    /// `sign_zome_call`, `import_key_seed`, ...) lives here — this plugin is a
    /// thin Tauri adapter, not a re-implementation.
    pub fn runtime(&self) -> &Runtime {
        &self.runtime
    }

    /// Bind a webview window (by its label) to an installed app, so that
    /// `app_request` IPC calls from that window are scoped to that app. This is
    /// the in-process replacement for the per-app websocket auth token: a window
    /// can only reach the app it was opened for.
    pub fn bind_window(&self, label: impl Into<String>, app_id: InstalledAppId) {
        self.window_apps
            .lock()
            .unwrap()
            .insert(label.into(), app_id);
    }

    fn app_id_for_window(&self, label: &str) -> Result<InstalledAppId> {
        self.window_apps
            .lock()
            .unwrap()
            .get(label)
            .cloned()
            .ok_or(Error::WindowNotBound)
    }

    /// Core of the `app_request` command: decode a msgpack-encoded App API
    /// request, dispatch it to the conductor scoped to the app bound to
    /// `window_label`, and return the msgpack-encoded App API response.
    ///
    /// The bytes are the same `{ type, value }` payloads the app websocket
    /// carries (holochain's `SerializedBytes` codec), so this is wire-compatible
    /// with `@holochain/client`'s Tauri transport. App-level failures come back
    /// as an encoded `AppResponse::Error`, matching the websocket interface.
    pub async fn app_request_bytes(
        &self,
        window_label: &str,
        request: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let app_id = self.app_id_for_window(window_label)?;
        let app_request: AppRequest =
            decode(&request).map_err(|e| Error::Serialization(e.to_string()))?;
        let response = self.runtime.handle_app_request(app_id, app_request).await?;
        encode(&response).map_err(|e| Error::Serialization(e.to_string()))
    }

    /// Build a webview window wired to the in-process conductor for `app_id`.
    ///
    /// By default this uses **direct Tauri IPC**: the window is bound to the app
    /// (so `app_request` calls are scoped to it), the app's signals are
    /// forwarded to it as [`EVENT_SIGNAL`], and `__HC_TAURI_HOLOCHAIN__` is
    /// injected so `@holochain/client` routes the App API through IPC with no
    /// loopback websocket. Set [`WindowOptions::use_app_websocket`] to fall back
    /// to the legacy `__HC_LAUNCHER_ENV__` websocket wiring instead.
    ///
    /// Call `.build()` on the returned builder to actually open the window.
    pub async fn main_window_builder(
        &self,
        label: impl Into<String>,
        app_id: String,
        options: WindowOptions,
    ) -> Result<WebviewWindowBuilder<'_, R, AppHandle<R>>> {
        let label: String = label.into();
        let url = options
            .url
            .unwrap_or_else(|| WebviewUrl::App("index.html".into()));

        let env_script = if options.use_app_websocket {
            // Legacy: attach an app websocket and point @holochain/client at it.
            let app_auth = self.runtime.ensure_app_websocket(app_id.clone()).await?;
            format!(
                r#"window.injectHolochainClientEnv("{}", {}, {:?});"#,
                app_id, app_auth.port, app_auth.authentication.token,
            )
        } else {
            // Direct: bind this window to the app, forward its signals, and
            // inject the env that routes the App API over Tauri IPC.
            self.bind_window(label.clone(), app_id.clone());
            self.spawn_signal_forwarder(label.clone(), app_id.clone())
                .await?;
            format!(r#"window.injectHolochainTauriEnv({app_id:?}, "holochain");"#)
        };

        let mut window_builder =
            WebviewWindowBuilder::new(&self.app_handle, label, url)
                .initialization_script(include_str!("../dist-js/holochain-env/index.min.js"))
                .initialization_script(env_script.as_str());

        if let Some(title) = options.title {
            window_builder = window_builder.title(title);
        }

        Ok(window_builder)
    }

    /// Subscribe to `app_id`'s signals and forward each to the `label` window as
    /// an [`EVENT_SIGNAL`] event carrying the msgpack-encoded conductor signal.
    /// The injected env's `subscribeSignals` bridge delivers these to
    /// `@holochain/client`. The task runs until the conductor's signal channel
    /// closes.
    async fn spawn_signal_forwarder(&self, label: String, app_id: InstalledAppId) -> Result<()> {
        let mut signals = self.runtime.subscribe_to_app_signals(app_id).await?;
        let app_handle = self.app_handle.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                match signals.recv().await {
                    Ok(signal) => match encode(&signal) {
                        Ok(bytes) => {
                            if let Err(e) = app_handle.emit_to(label.as_str(), EVENT_SIGNAL, bytes) {
                                log::error!("Failed to forward signal to window {label}: {e:?}");
                            }
                        }
                        Err(e) => log::error!("Failed to encode signal: {e}"),
                    },
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        log::warn!("Signal forwarder for window {label} lagged; dropped {n}");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        });
        Ok(())
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
        .invoke_handler(tauri::generate_handler![
            commands::sign_zome_call,
            commands::app_request
        ])
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
                            window_apps: Arc::new(Mutex::new(HashMap::new())),
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
