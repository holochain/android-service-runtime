//! Minimal example (desktop + Android) for the in-process
//! `tauri-plugin-holochain`.
//!
//! On startup it boots an in-process Holochain conductor (via the plugin),
//! installs + enables the bundled `forum.happ` fixture, then opens a window
//! bound to the app over direct Tauri IPC. The webview (`ui/index.html`) reads
//! the injected `__HC_TAURI_HOLOCHAIN__` env and connects with
//! `@holochain/client` — no loopback websocket.

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use holochain::prelude::{AppBundleSource, InstallAppPayload};
// `Manager` brings `app.path()` into scope — needed by every mobile arm of
// `data_dir` except the iOS simulator, which uses a fixed absolute path.
#[cfg(all(mobile, not(all(target_os = "ios", target_abi = "sim"))))]
use tauri::Manager;
use tauri::{AppHandle, Listener};
use tauri_plugin_holochain::{
    vec_to_locked, HolochainExt, HolochainPluginConfig, NetworkConfig, WindowOptions, EVENT_READY,
    EVENT_SETUP_FAILED,
};

const APP_ID: &str = "forum";
const HAPP_BUNDLE: &[u8] = include_bytes!("../../../../crates/runtime/fixtures/forum.happ");

/// Desktop keeps the historical throwaway temp dir (the integration test relies
/// on a fresh conductor per run); Android uses the per-app data dir. iOS cannot
/// use its app data dir at all — see below.
fn data_dir(app: &AppHandle) -> PathBuf {
    #[cfg(desktop)]
    {
        let _ = app;
        std::env::temp_dir().join("holochain-runtime-example")
    }
    #[cfg(target_os = "android")]
    {
        app.path()
            .app_data_dir()
            .expect("no app data dir on this platform")
            .join("holochain")
    }
    // iOS has a hard path-length budget the other platforms don't.
    //
    // Holochain's "in-process" lair keystore is really a StandaloneServer bound
    // to a unix domain socket at `<data_root>/socket`, and AF_UNIX paths are
    // capped at ~104 bytes (SUN_LEN). Lair also canonicalizes the root, and on
    // iOS `/var` -> `/private/var` costs another 8 bytes. Budget for the
    // socket, measured with a real 36-char container UUID:
    //
    //   device    <container>/hc/socket                                 94 B  ok
    //   device    <container>/Documents/hc/socket                      104 B  fails
    //   device    app_data_dir()/holochain/socket                      158 B  fails
    //   simulator <container>/hc/socket                                172 B  fails
    //
    // So on device nothing with the bundle id in it fits (the id alone is 28
    // bytes) and `app_data_dir()` is hopeless; the container root plus a
    // 2-char subdir clears it with ~10 bytes to spare. In the simulator the
    // container is 162 bytes on its own, so nothing under it can ever work —
    // but simulator apps can write outside their container, so use a short
    // absolute path there.
    //
    // Neither arm is shippable. 10 bytes of headroom is not a design, and the
    // simulator arm leaves data unsandboxed and unbacked-up. The real fix is
    // upstream — holochain already has a socket-free in-proc keystore path it
    // chooses not to use here. See docs/ios-test-build-plan.md §4.0.
    //
    // UNTESTED on a physical device: no signing team was available.
    #[cfg(all(target_os = "ios", not(target_abi = "sim")))]
    {
        app.path()
            .home_dir()
            .expect("no home dir on iOS")
            .join("hc")
    }
    #[cfg(all(target_os = "ios", target_abi = "sim"))]
    {
        let _ = app;
        PathBuf::from("/tmp/hcex")
    }
}

/// Called back by the webview UI so its `@holochain/client` results are visible
/// in stdout even when running headless (no display).
#[tauri::command]
fn report(step: String, ok: bool, detail: String) {
    if ok {
        log::info!("[ui] {step}: OK — {detail}");
    } else {
        log::error!("[ui] {step}: FAIL — {detail}");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Warn)
                .level_for("holochain_runtime_example_lib", log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![report])
        .setup(|app| {
            let handle = app.handle().clone();

            app.handle().listen(EVENT_SETUP_FAILED, |event| {
                log::error!("holochain setup failed: {}", event.payload());
            });

            app.handle().listen(EVENT_READY, move |_| {
                log::info!("holochain conductor ready; installing app + opening window");
                let handle = handle.clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(e) = open_main_window(handle).await {
                        log::error!("failed to open main window: {e}");
                    }
                });
            });

            // Registered here (not on the builder) because the mobile data dir
            // comes from the app's path resolver, which needs a live handle.
            let holochain_data_dir = data_dir(app.handle());
            app.handle().plugin(tauri_plugin_holochain::init(
                vec_to_locked(vec![]),
                HolochainPluginConfig::new(holochain_data_dir, NetworkConfig::default()),
            ))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn open_main_window(handle: AppHandle) -> Result<(), Box<dyn Error>> {
    let plugin = handle.holochain()?;

    // Install + enable the forum hApp and ensure its app websocket exists.
    plugin
        .runtime()
        .setup_app(
            InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_BUNDLE.to_vec().into()),
                agent_key: None,
                installed_app_id: Some(APP_ID.into()),
                network_seed: None,
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
                restore_from_dht: false,
            },
            true,
        )
        .await?;

    // Open the window wired to the conductor (injects __HC_LAUNCHER_ENV__).
    plugin
        .main_window_builder(
            "main",
            Some(APP_ID.to_string()),
            WindowOptions {
                title: Some("Holochain Runtime Example".into()),
                ..Default::default()
            },
        )
        .await?
        .build()?;

    log::info!("main window opened");
    Ok(())
}
