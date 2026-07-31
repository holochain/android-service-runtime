//! Minimal **desktop** example for the in-process `tauri-plugin-holochain`.
//!
//! On startup it boots an in-process Holochain conductor (via the plugin),
//! installs + enables the bundled `forum.happ` fixture, then opens a window
//! wired to the conductor's app websocket. The webview (`ui/index.html`) reads
//! the injected `__HC_LAUNCHER_ENV__` and connects with `@holochain/client`.

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use holochain::prelude::{AppBundleSource, InstallAppPayload};
use tauri::{AppHandle, Listener};
use tauri_plugin_holochain::{
    vec_to_locked, HolochainExt, HolochainPluginConfig, NetworkConfig, WindowOptions, EVENT_READY,
    EVENT_SETUP_FAILED,
};

const APP_ID: &str = "forum";
const HAPP_BUNDLE: &[u8] = include_bytes!("../../../../crates/runtime/fixtures/forum.happ");

fn data_dir() -> PathBuf {
    std::env::temp_dir().join("holochain-runtime-example")
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

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Warn)
                .level_for("holochain_runtime_example_lib", log::LevelFilter::Info)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![report])
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]),
            HolochainPluginConfig::new(data_dir(), NetworkConfig::default()),
        ))
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
