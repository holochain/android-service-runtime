//! Integration test (Approach A): prove the plugin boots a real Holochain
//! conductor *inside a Tauri app* and exposes a live, usable conductor.
//!
//! This drives the plugin exactly as a real app would: build a Tauri app with
//! the plugin, wait for `holochain://ready`, then install + enable the forum
//! fixture through the plugin's runtime and attach an app interface. It does not
//! open a webview — the full webview + `@holochain/client` path is covered by
//! the desktop example app (Approach B).

use std::collections::HashMap;
use std::time::Duration;

use holochain::prelude::{AppBundleSource, InstallAppPayload};
use holochain_types::prelude::AppStatus;
use tauri::test::{mock_builder, mock_context, noop_assets};
use tauri_plugin_holochain::{vec_to_locked, HolochainExt, HolochainPluginConfig, NetworkConfig};
use tempfile::TempDir;
use uuid::Uuid;

const HAPP_FIXTURE: &[u8] = include_bytes!("../../runtime/fixtures/forum.happ");
const APP_ID: &str = "forum";

#[test]
fn plugin_boots_conductor_in_tauri_app() {
    let tmp = TempDir::new().unwrap();

    // Build a real Tauri app (mock runtime) with our plugin installed.
    let app = mock_builder()
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]),
            HolochainPluginConfig::new(tmp.path().to_path_buf(), NetworkConfig::default()),
        ))
        .build(mock_context(noop_assets()))
        .expect("failed to build mock tauri app");

    // Everything runs on Tauri's async runtime (where the plugin spawned the
    // conductor boot), avoiding cross-runtime issues.
    tauri::async_runtime::block_on(async move {
        // Wait for the conductor to finish starting (the plugin emits
        // `holochain://ready` and manages its state at that point).
        let mut waited = Duration::ZERO;
        let step = Duration::from_millis(200);
        let plugin = loop {
            if let Ok(plugin) = app.holochain() {
                break plugin;
            }
            assert!(
                waited < Duration::from_secs(60),
                "conductor did not become ready within 60s"
            );
            tokio::time::sleep(step).await;
            waited += step;
        };

        let runtime = plugin.runtime();

        // Install + enable the forum hApp through the in-process conductor.
        runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: None,
                installed_app_id: Some(APP_ID.into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
            })
            .await
            .expect("install_app failed");
        runtime.enable_app(APP_ID.into()).await.expect("enable_app failed");

        // Attach an app interface — this is the websocket the plugin injects
        // into a webview. A real bound port proves the endpoint exists.
        let app_auth = runtime
            .ensure_app_websocket(APP_ID.into())
            .await
            .expect("ensure_app_websocket failed");
        assert!(app_auth.port > 0, "expected a bound app interface port");
        assert!(
            !app_auth.authentication.token.is_empty(),
            "expected a non-empty app auth token"
        );

        // The app is installed and enabled through the plugin.
        let apps = runtime.list_apps().await.expect("list_apps failed");
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].installed_app_id, APP_ID);
        assert_eq!(apps[0].status, AppStatus::Enabled);
    });
}
