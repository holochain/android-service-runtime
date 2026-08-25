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

#[cfg(any(test, feature = "test-utils"))]
pub mod test_support;

pub use error::{Error, Result};

// Re-export the native config type consumers build, and the runtime itself.
pub use holochain::conductor::config::NetworkConfig;
pub use holochain_conductor_runtime::Runtime;
pub use holochain_conductor_runtime::{ConductorError, RuntimeError};
// hc-auth: re-export the module and its config/status types so consumers can
// build a `HcAuthConfig` and read `HcAuthStatus` without depending on the
// runtime crate directly.
pub use holochain_conductor_runtime::hc_auth;
pub use holochain_conductor_runtime::{HcAuthConfig, HcAuthStatus};
pub use lair_keystore_api::types::SharedLockedArray;

use holochain::conductor::api::AppRequest;
use holochain::prelude::{decode, encode, InstalledAppId};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use tokio::sync::broadcast;

use sodoken::LockedArray;
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Emitter, Manager, RunEvent, Runtime as TauriRuntime, WebviewUrl,
    WebviewWindowBuilder, WindowEvent,
};

/// Emitted on the app handle once the conductor is up and [`HolochainExt::holochain`]
/// is ready to use.
pub const EVENT_READY: &str = "holochain://ready";
/// Emitted on the app handle once the lair keystore is unlocked and the device
/// key is available, before the conductor has finished coming up. Consumers can
/// drive UX (e.g. an hc-auth verification screen) during the conductor-up phase.
pub const EVENT_LAIR_READY: &str = "holochain://lair-ready";
/// Emitted on the app handle if the conductor fails to start. The payload is the
/// error string.
pub const EVENT_SETUP_FAILED: &str = "holochain://setup-failed";
/// Emitted to a direct-mode webview window with the msgpack-encoded conductor
/// [`holochain_types::signal::Signal`] for the app the window is bound to. The
/// injected env bridges this to `@holochain/client`'s Tauri transport.
pub const EVENT_SIGNAL: &str = "holochain://signal";

/// Emitted to a window when [`HolochainPlugin::rebind_window`] changes the app it
/// is bound to — **without recreating the OS window**. The payload is a
/// [`ReboundEvent`]: a monotonic `seq` plus the new `app_id` (`null` when the
/// window is unbound). The injected env applies it only when `seq` exceeds the
/// last it applied — so an out-of-order delivery can't leave the UI on a stale
/// app — then updates `__HC_TAURI_HOLOCHAIN__.INSTALLED_APP_ID` and the SPA
/// re-connects the App API to the new app.
pub const EVENT_REBOUND: &str = "holochain://rebound";

/// Payload of [`EVENT_REBOUND`]. `seq` is a per-plugin monotonic counter so the
/// injected env can drop a stale (out-of-order) rebound; `app_id` is the new
/// binding, or `None` when the window is unbound.
#[derive(Clone, serde::Serialize)]
pub struct ReboundEvent {
    pub seq: u64,
    pub app_id: Option<InstalledAppId>,
}

/// Configuration for the in-process Holochain conductor.
#[derive(Clone)]
pub struct HolochainPluginConfig {
    /// Directory where conductor + keystore data is stored.
    pub data_dir: PathBuf,
    /// Native holochain network config (bootstrap / signal / relay / ...).
    pub network: NetworkConfig,
    /// If set, run the hc-auth flow at boot and inject the material into the
    /// network config (authenticated mode). See [`HolochainPluginConfig::with_hc_auth`].
    pub hc_auth: Option<HcAuthConfig>,
    /// If set (32 bytes), import as the device seed before boot (agent-key
    /// restore). See [`HolochainPluginConfig::with_pending_import_seed`].
    pub pending_import_seed: Option<Vec<u8>>,
}

impl HolochainPluginConfig {
    pub fn new(data_dir: PathBuf, network: NetworkConfig) -> Self {
        Self {
            data_dir,
            network,
            hc_auth: None,
            pending_import_seed: None,
        }
    }

    /// Enable authenticated mode: the hc-auth flow runs at boot and its material
    /// is injected into the network config.
    pub fn with_hc_auth(mut self, hc_auth: HcAuthConfig) -> Self {
        self.hc_auth = Some(hc_auth);
        self
    }

    /// Import `seed` (32 bytes) as the device seed before boot, so the first
    /// install derives this identity (agent-key restore).
    pub fn with_pending_import_seed(mut self, seed: Vec<u8>) -> Self {
        self.pending_import_seed = Some(seed);
        self
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

/// Outcome of the conductor boot, held in place of a bare `Option<Runtime>` so a
/// failure is remembered rather than looking the same as a boot still in flight.
enum BootState {
    /// No boot has finished: either none was started ([`init_deferred`]) or one
    /// is still running.
    NotStarted,
    Ready(Runtime),
    /// The last boot attempt failed with this cause. Kept even though
    /// [`EVENT_SETUP_FAILED`] carries the same string: that event is transient,
    /// so anything not already listening when it fires cannot recover the cause.
    Failed(String),
}

/// Access to the running in-process Holochain conductor from the Tauri app.
///
/// The plugin may be registered before the conductor boots ([`init_deferred`]),
/// so the runtime is populated late by [`HolochainPlugin::start`]. Until then
/// [`HolochainPlugin::runtime`] panics and [`HolochainPlugin::try_runtime`]
/// reports [`Error::NotReady`], or [`Error::SetupFailed`] once a boot has failed.
pub struct HolochainPlugin<R: TauriRuntime> {
    /// What the conductor boot has produced so far. Behind an `RwLock` so a hot
    /// restart can swap in a new runtime on the same lair without re-registering
    /// the plugin (see `swap_runtime`).
    boot_state: RwLock<BootState>,
    /// Boot config, consumed by [`HolochainPlugin::start`]. Kept (not taken) so a
    /// failed unlock can be retried with a different passphrase.
    pending_config: Mutex<Option<HolochainPluginConfig>>,
    /// Serializes [`HolochainPlugin::start`] so two concurrent unlock attempts
    /// can't both build a runtime.
    start_lock: tokio::sync::Mutex<()>,
    app_handle: AppHandle<R>,
    /// Maps a webview window label to the app it is bound to. Populated by
    /// [`HolochainPlugin::main_window_builder`] / [`HolochainPlugin::bind_window`].
    /// The `app_request` command uses this to scope each request to the app the
    /// calling window was opened for — replacing the per-app websocket token.
    window_apps: Arc<Mutex<HashMap<String, InstalledAppId>>>,
    /// Per-window signal-forwarder task handles, so a rebind (or the re-bind done
    /// by [`swap_runtime`]) can abort the previous forwarder before starting the
    /// new one — otherwise the old app's signals would keep arriving at the window.
    window_forwarders: Arc<Mutex<HashMap<String, tauri::async_runtime::JoinHandle<()>>>>,
    /// Monotonic rebind counter — rides each [`EVENT_REBOUND`] as its `seq` so the
    /// injected env can drop a stale (out-of-order) rebound rather than leave the
    /// UI on a different app than `app_request` routes to.
    rebind_seq: AtomicU64,
}

impl<R: TauriRuntime> HolochainPlugin<R> {
    /// The underlying conductor runtime. The full lifecycle API
    /// (`install_app`, `enable_app`, `setup_app`, `ensure_app_websocket`,
    /// `sign_zome_call`, `import_key_seed`, ...) lives here — this plugin is a
    /// thin Tauri adapter, not a re-implementation.
    ///
    /// Returns a cheap clone of the runtime handle (it is internally `Arc`-backed).
    /// Panics if called before the conductor has started; gate on [`EVENT_READY`]
    /// (or use [`HolochainExt::holochain`] only after it fires).
    pub fn runtime(&self) -> Runtime {
        self.try_runtime()
            .expect("holochain runtime not started yet; wait for EVENT_READY before calling runtime()")
    }

    /// Like [`HolochainPlugin::runtime`] but returns an error instead of
    /// panicking: [`Error::NotReady`] while no boot has completed, or
    /// [`Error::SetupFailed`] carrying the cause once one has failed.
    pub fn try_runtime(&self) -> Result<Runtime> {
        match &*self.boot_state.read().unwrap() {
            BootState::Ready(runtime) => Ok(runtime.clone()),
            BootState::Failed(cause) => Err(Error::SetupFailed(cause.clone())),
            BootState::NotStarted => Err(Error::NotReady),
        }
    }

    /// Boot the conductor with `passphrase` using the config supplied at
    /// registration ([`init`] / [`init_deferred`]).
    ///
    /// This is the deferred-boot entry point: [`init_deferred`] registers the
    /// plugin without booting, and the host app calls `start(passphrase)` from a
    /// command once it has collected the (possibly late) passphrase — e.g. a
    /// user-typed lair password. See [`HolochainPlugin::start_with_config`] for
    /// the variant that boots against a freshly-built config (the host needs
    /// this when the config depends on per-unlock state, e.g. hc-auth or an
    /// imported seed).
    pub async fn start(&self, passphrase: SharedLockedArray) -> Result<()> {
        let config = self
            .pending_config
            .lock()
            .unwrap()
            .clone()
            .ok_or(Error::NotReady)?;
        self.start_with_config(passphrase, config).await
    }

    /// Boot the conductor with `passphrase` against `config`, unlocking (or
    /// creating) the lair keystore and bringing the conductor up. Emits
    /// [`EVENT_LAIR_READY`] once the keystore is available and [`EVENT_READY`]
    /// once the conductor is up.
    ///
    /// Retryable: a failed boot records its cause (reported by
    /// [`HolochainPlugin::try_runtime`] as [`Error::SetupFailed`]) and leaves the
    /// conductor unstarted, so a subsequent call with a corrected
    /// passphrase/config can succeed. Returns [`Error::AlreadyStarted`] if the
    /// conductor is already running.
    pub async fn start_with_config(
        &self,
        passphrase: SharedLockedArray,
        config: HolochainPluginConfig,
    ) -> Result<()> {
        // Serialize concurrent unlock attempts and re-check under the lock.
        let _guard = self.start_lock.lock().await;
        {
            let mut state = self.boot_state.write().unwrap();
            if matches!(*state, BootState::Ready(_)) {
                return Err(Error::AlreadyStarted);
            }
            // This attempt supersedes the last one's cause: while it is in flight
            // the plugin is not-started, not still-failed.
            *state = BootState::NotStarted;
        }

        let result = self.boot(passphrase, config).await;
        if let Err(e) = &result {
            let mut state = self.boot_state.write().unwrap();
            // `swap_runtime` may have installed a live runtime while this attempt
            // was in flight, and failing it must not drop that runtime.
            if !matches!(*state, BootState::Ready(_)) {
                *state = BootState::Failed(e.to_string());
            }
        }
        result
    }

    async fn boot(
        &self,
        passphrase: SharedLockedArray,
        config: HolochainPluginConfig,
    ) -> Result<()> {
        // Spawn lair, (optionally) run the hc-auth flow, then bring the conductor
        // up against that lair — see `Runtime::new_with_boot_config`. The
        // controllable boot collapses to a single conductor start; the keystore
        // is unlocked and the device key exists before `EVENT_LAIR_READY`.
        let runtime = Runtime::new_with_boot_config(
            passphrase,
            holochain_conductor_runtime::RuntimeBootConfig {
                data_root_path: config.data_dir,
                network: config.network,
                hc_auth: config.hc_auth,
                pending_import_seed: config.pending_import_seed,
            },
        )
        .await?;

        if let Err(e) = self.app_handle.emit(EVENT_LAIR_READY, ()) {
            log::error!("Failed to emit {EVENT_LAIR_READY}: {e:?}");
        }

        *self.boot_state.write().unwrap() = BootState::Ready(runtime);

        if let Err(e) = self.app_handle.emit(EVENT_READY, ()) {
            log::error!("Failed to emit {EVENT_READY}: {e:?}");
        }
        Ok(())
    }

    /// Swap in a new conductor runtime (e.g. after an hc-auth hot restart) and
    /// re-bind existing webview windows to it.
    ///
    /// The window→app map is preserved (the apps persist on the same lair), so we
    /// only need to re-spawn each bound window's signal forwarder against the new
    /// runtime — signal subscriptions are per-runtime and the old conductor's
    /// channels are gone after the restart.
    pub async fn swap_runtime(&self, new: Runtime) -> Result<()> {
        *self.boot_state.write().unwrap() = BootState::Ready(new);

        let bindings: Vec<(String, InstalledAppId)> = self
            .window_apps
            .lock()
            .unwrap()
            .iter()
            .map(|(label, app_id)| (label.clone(), app_id.clone()))
            .collect();

        for (label, app_id) in bindings {
            if let Err(e) = self.spawn_signal_forwarder(label.clone(), app_id).await {
                log::error!("Failed to re-spawn signal forwarder for window {label}: {e:?}");
            }
        }
        Ok(())
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

    /// Rebind an existing webview window to a different installed app — or unbind
    /// it — **without recreating the OS window**. Updates the `app_request`
    /// routing, swaps the window's signal forwarder, and emits [`EVENT_REBOUND`]
    /// to the window so the injected env can update
    /// `__HC_TAURI_HOLOCHAIN__.INSTALLED_APP_ID` and the SPA can re-connect the App
    /// API to the new app.
    ///
    /// `app_id = Some(..)` (re)binds to that app; `None` unbinds the window
    /// (app-less / dashboard). This is the in-process, no-flicker alternative to
    /// destroying and rebuilding the window to switch which app it talks to — the
    /// WebDriver session and window state survive, and there is no boot flash.
    pub async fn rebind_window(
        &self,
        label: impl Into<String>,
        app_id: Option<InstalledAppId>,
    ) -> Result<()> {
        let label: String = label.into();
        // Stamp each rebound with a monotonic seq so the injected env can drop a
        // stale (out-of-order) one rather than leave the UI on the wrong app.
        let seq = self.rebind_seq.fetch_add(1, Ordering::Relaxed) + 1;
        match app_id {
            Some(app_id) => {
                // Spawn the new forwarder first — it is the fallible step (its
                // subscribe can fail mid-restart). Flip routing only once it
                // succeeds, so a failed rebind leaves the window on its previous
                // app instead of routing app_request to an app whose signal
                // stream never started.
                self.spawn_signal_forwarder(label.clone(), app_id.clone())
                    .await?;
                self.bind_window(label.clone(), app_id.clone());
                self.app_handle.emit_to(
                    label.as_str(),
                    EVENT_REBOUND,
                    ReboundEvent {
                        seq,
                        app_id: Some(app_id),
                    },
                )?;
            }
            None => {
                self.drop_window(&label);
                self.app_handle.emit_to(
                    label.as_str(),
                    EVENT_REBOUND,
                    ReboundEvent { seq, app_id: None },
                )?;
            }
        }
        Ok(())
    }

    /// The app a window is currently bound to, or `None` if it is unbound.
    pub fn bound_app(&self, label: &str) -> Option<InstalledAppId> {
        self.window_apps.lock().unwrap().get(label).cloned()
    }

    /// Drop a window's routing and abort its signal forwarder. Used when a
    /// window is unbound (rebind to `None`) or destroyed.
    fn drop_window(&self, label: &str) {
        self.window_apps.lock().unwrap().remove(label);
        if let Some(prev) = self.window_forwarders.lock().unwrap().remove(label) {
            prev.abort();
        }
    }

    fn app_id_for_window(&self, label: &str) -> Result<InstalledAppId> {
        self.bound_app(label).ok_or(Error::WindowNotBound)
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
        let response = self
            .try_runtime()?
            .handle_app_request(app_id, app_request)
            .await?;
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
        app_id: Option<String>,
        options: WindowOptions,
    ) -> Result<WebviewWindowBuilder<'_, R, AppHandle<R>>> {
        let label: String = label.into();
        let url = options
            .url
            .unwrap_or_else(|| WebviewUrl::App("index.html".into()));

        let env_script = if options.use_app_websocket {
            // Legacy: attach an app websocket and point @holochain/client at it.
            // This path requires a bound app.
            let app_id = app_id.ok_or(Error::WindowNotBound)?;
            let app_auth = self.try_runtime()?.ensure_app_websocket(app_id.clone()).await?;
            format!(
                r#"window.injectHolochainClientEnv("{}", {}, {:?});"#,
                app_id, app_auth.port, app_auth.authentication.token,
            )
        } else {
            // Direct: inject the IPC env (+ the rebound listener). If an app is
            // given, bind the window and forward its signals; `None` opens an
            // app-less window (dashboard) that `rebind_window` can bind later
            // without recreating the OS window.
            if let Some(app_id) = &app_id {
                self.bind_window(label.clone(), app_id.clone());
                self.spawn_signal_forwarder(label.clone(), app_id.clone())
                    .await?;
            }
            let injected = app_id.unwrap_or_default();
            format!(r#"window.injectHolochainTauriEnv({injected:?}, "holochain");"#)
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
        let mut signals = self.try_runtime()?.subscribe_to_app_signals(app_id).await?;
        let app_handle = self.app_handle.clone();
        let task_label = label.clone();
        let handle = tauri::async_runtime::spawn(async move {
            loop {
                match signals.recv().await {
                    Ok(signal) => match encode(&signal) {
                        Ok(bytes) => {
                            if let Err(e) =
                                app_handle.emit_to(task_label.as_str(), EVENT_SIGNAL, bytes)
                            {
                                log::error!("Failed to forward signal to window {task_label}: {e:?}");
                            }
                        }
                        Err(e) => log::error!("Failed to encode signal: {e}"),
                    },
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        log::warn!("Signal forwarder for window {task_label} lagged; dropped {n}");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        });
        // Replace (and abort) any prior forwarder for this window — a rebind or a
        // swap_runtime re-bind must not leave the old app's forwarder running.
        if let Some(prev) = self.window_forwarders.lock().unwrap().insert(label, handle) {
            prev.abort();
        }
        Ok(())
    }
}

/// Extension trait giving Tauri's `App`/`AppHandle`/`Window` access to the
/// in-process Holochain conductor.
pub trait HolochainExt<R: TauriRuntime> {
    /// Access the Holochain plugin handle. Available as soon as the plugin is
    /// registered — including before the conductor boots, so `init_deferred`
    /// consumers can call [`HolochainPlugin::start`]. The handle being available
    /// does **not** mean the runtime is: gate conductor use on [`EVENT_READY`]
    /// (or [`HolochainPlugin::try_runtime`]). Returns [`Error::NotReady`] only
    /// when the plugin isn't registered.
    fn holochain(&self) -> Result<&HolochainPlugin<R>>;
}

impl<R: TauriRuntime, T: Manager<R>> HolochainExt<R> for T {
    fn holochain(&self) -> Result<&HolochainPlugin<R>> {
        self.try_state::<HolochainPlugin<R>>()
            .map(|state| state.inner())
            .ok_or(Error::NotReady)
    }
}

/// Register the plugin and manage a [`HolochainPlugin`] without booting the
/// conductor. Shared by [`init`] (which then boots immediately) and
/// [`init_deferred`] (which waits for the host to call
/// [`HolochainPlugin::start`]).
fn plugin_builder<R: TauriRuntime>(
    config: HolochainPluginConfig,
    on_setup: impl Fn(&AppHandle<R>) + Send + Sync + 'static,
) -> TauriPlugin<R> {
    Builder::new("holochain")
        .invoke_handler(tauri::generate_handler![
            commands::sign_zome_call,
            commands::sign_payload,
            commands::app_request
        ])
        .setup(move |app, _api| {
            app.manage(HolochainPlugin {
                boot_state: RwLock::new(BootState::NotStarted),
                pending_config: Mutex::new(Some(config.clone())),
                start_lock: tokio::sync::Mutex::new(()),
                app_handle: app.clone(),
                window_apps: Arc::new(Mutex::new(HashMap::new())),
                window_forwarders: Arc::new(Mutex::new(HashMap::new())),
                rebind_seq: AtomicU64::new(0),
            });
            on_setup(app);
            Ok(())
        })
        // Prune a window's routing + signal forwarder when it is destroyed, so
        // the maps don't grow unbounded and a closed window's forwarder task is
        // aborted rather than left running until its app's signal channel closes.
        .on_event(|app_handle, event| {
            if let RunEvent::WindowEvent {
                label,
                event: WindowEvent::Destroyed,
                ..
            } = event
            {
                if let Ok(plugin) = app_handle.holochain() {
                    plugin.drop_window(label);
                }
            }
        })
        .build()
}

/// Initialize the plugin and boot the conductor immediately with `passphrase`.
///
/// The conductor is booted asynchronously so the Tauri app can show a
/// splashscreen while it starts. [`EVENT_READY`] is emitted once
/// [`HolochainExt::holochain`] is usable; [`EVENT_SETUP_FAILED`] is emitted on
/// failure. Use [`init_deferred`] instead if the passphrase isn't known at
/// registration time (e.g. a user-typed lair password).
pub fn init<R: TauriRuntime>(
    passphrase: SharedLockedArray,
    config: HolochainPluginConfig,
) -> TauriPlugin<R> {
    plugin_builder(config, move |app| {
        let app_handle = app.clone();
        let passphrase = passphrase.clone();
        tauri::async_runtime::spawn(async move {
            // `holochain()` borrows manager-lifetime state, so the reference is
            // valid across the await (app_handle outlives the task).
            let result = match app_handle.holochain() {
                Ok(plugin) => plugin.start(passphrase).await,
                Err(e) => Err(e),
            };
            if let Err(e) = result {
                log::error!("Holochain conductor setup failed: {e:?}");
                let _ = app_handle.emit(EVENT_SETUP_FAILED, e.to_string());
            }
        });
    })
}

/// Initialize the plugin **without** booting the conductor.
///
/// The plugin is registered and [`HolochainExt::holochain`] becomes available
/// immediately, but no conductor runs until the host calls
/// [`HolochainPlugin::start`] with a passphrase (typically from a Tauri command
/// after collecting a user-typed lair password). No [`EVENT_READY`] is emitted
/// until that boot succeeds.
pub fn init_deferred<R: TauriRuntime>(config: HolochainPluginConfig) -> TauriPlugin<R> {
    plugin_builder(config, |_app| {})
}
