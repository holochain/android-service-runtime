//! Integration tests (Approach A): prove the plugin boots a real Holochain
//! conductor *inside a Tauri app* and serves it both ways — the legacy app
//! websocket and the new in-process `app_request` IPC path.
//!
//! These drive the plugin as a real app would: build a Tauri app with the
//! plugin and wait for `holochain://ready`. They do not open a webview — the
//! full webview + `@holochain/client` path is covered by the desktop example
//! app (Approach B).

use std::collections::HashMap;
use std::time::Duration;

use holochain::conductor::api::CellInfo::Provisioned;
use holochain::conductor::api::{AppRequest, AppResponse, ProvisionedCell};
use holochain::prelude::{
    decode, encode, AppBundleSource, ExternIO, InstallAppPayload, ZomeCallParams,
};
use holochain_types::prelude::{AppStatus, Link, Nonce256Bits, Timestamp};
use tauri::test::{mock_builder, mock_context, noop_assets};
use tauri_plugin_holochain::{
    vec_to_locked, Error, HolochainExt, HolochainPluginConfig, NetworkConfig,
};
use tempfile::TempDir;
use uuid::Uuid;

const HAPP_FIXTURE: &[u8] = include_bytes!("../../runtime/fixtures/forum.happ");
const APP_ID: &str = "forum";

/// Build a mock Tauri app with the plugin installed.
fn build_app(tmp: &TempDir) -> tauri::App<tauri::test::MockRuntime> {
    mock_builder()
        .plugin(tauri_plugin_holochain::init(
            vec_to_locked(vec![]),
            HolochainPluginConfig::new(tmp.path().to_path_buf(), NetworkConfig::default()),
        ))
        .build(mock_context(noop_assets()))
        .expect("failed to build mock tauri app")
}

/// Wait until the plugin has finished booting the conductor.
///
/// The plugin state is `manage()`d at setup (so `holochain()` resolves
/// immediately, which the deferred-boot flow relies on); the conductor is then
/// booted asynchronously. Readiness is therefore signalled by `try_runtime()`
/// succeeding — the same point at which `holochain://ready` is emitted — not by
/// `holochain()` alone.
async fn wait_for_ready<R: tauri::Runtime>(app: &tauri::App<R>) {
    let mut waited = Duration::ZERO;
    let step = Duration::from_millis(200);
    while app.holochain().and_then(|h| h.try_runtime()).is_err() {
        assert!(
            waited < Duration::from_secs(60),
            "conductor did not become ready within 60s"
        );
        tokio::time::sleep(step).await;
        waited += step;
    }
}

async fn install_and_enable_forum(
    runtime: &tauri_plugin_holochain::Runtime,
) -> holochain::conductor::api::AppInfo {
    let app_info = runtime
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
    runtime
        .enable_app(APP_ID.into())
        .await
        .expect("enable_app failed");
    app_info
}

#[test]
fn plugin_boots_conductor_in_tauri_app() {
    let tmp = TempDir::new().unwrap();
    let app = build_app(&tmp);

    // Everything runs on Tauri's async runtime (where the plugin spawned the
    // conductor boot), avoiding cross-runtime issues.
    tauri::async_runtime::block_on(async move {
        wait_for_ready(&app).await;
        let runtime = app.holochain().unwrap().runtime();

        install_and_enable_forum(&runtime).await;

        // Attach an app interface — this is the websocket the legacy injection
        // wires a webview to. A real bound port proves the endpoint exists.
        let app_auth = runtime
            .ensure_app_websocket(APP_ID.into())
            .await
            .expect("ensure_app_websocket failed");
        assert!(app_auth.port > 0, "expected a bound app interface port");
        assert!(
            !app_auth.authentication.token.is_empty(),
            "expected a non-empty app auth token"
        );

        let apps = runtime.list_apps().await.expect("list_apps failed");
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].installed_app_id, APP_ID);
        assert_eq!(apps[0].status, AppStatus::Enabled);
    });
}

/// The `app_request` IPC path serves the App API in-process: a window bound to
/// an app can fetch app info and make a (signed) zome call, with no app
/// websocket attached. The bytes use holochain's `SerializedBytes` codec —
/// identical to what `@holochain/client`'s Tauri transport sends.
#[test]
fn app_request_serves_app_api_in_process() {
    let tmp = TempDir::new().unwrap();
    let app = build_app(&tmp);

    tauri::async_runtime::block_on(async move {
        wait_for_ready(&app).await;
        let plugin = app.holochain().unwrap();
        let runtime = plugin.runtime();

        let app_info = install_and_enable_forum(&runtime).await;

        // main_window_builder does this in real use; bind a label directly here.
        plugin.bind_window("main", APP_ID.into());

        // AppInfo over the IPC codec.
        let resp_bytes = plugin
            .app_request_bytes("main", encode(&AppRequest::AppInfo).unwrap())
            .await
            .expect("app_request AppInfo failed");
        let resp: AppResponse = decode(&resp_bytes).unwrap();
        let AppResponse::AppInfo(Some(info)) = resp else {
            panic!("expected AppResponse::AppInfo(Some(_)), got {resp:?}");
        };
        assert_eq!(info.installed_app_id, APP_ID);

        // A signed CallZome over the IPC codec round-trips to an empty post list.
        // Role name is "forum"; the coordinator zome inside it is "posts".
        let Provisioned(ProvisionedCell { cell_id, .. }) =
            app_info.cell_info.get("forum").unwrap().first().unwrap()
        else {
            panic!("App Info has no CellId")
        };
        let signed = runtime
            .sign_zome_call(ZomeCallParams {
                provenance: cell_id.agent_pubkey().clone(),
                cell_id: cell_id.clone(),
                zome_name: "posts".into(),
                fn_name: "get_all_posts".into(),
                cap_secret: None,
                payload: ExternIO::encode(()).unwrap(),
                nonce: Nonce256Bits::from([0; 32]),
                expires_at: Timestamp(Timestamp::now().as_micros() + 60_000_000),
            })
            .await
            .unwrap();
        let resp_bytes = plugin
            .app_request_bytes(
                "main",
                encode(&AppRequest::CallZome(Box::new(signed))).unwrap(),
            )
            .await
            .expect("app_request CallZome failed");
        let resp: AppResponse = decode(&resp_bytes).unwrap();
        let AppResponse::ZomeCalled(io) = resp else {
            panic!("expected AppResponse::ZomeCalled, got {resp:?}");
        };
        let posts: Vec<Link> = io.decode().unwrap();
        assert!(posts.is_empty());

        // A request from an unbound window is refused — a window can only reach
        // the app it was opened for.
        let unbound = plugin
            .app_request_bytes("not-bound", encode(&AppRequest::AppInfo).unwrap())
            .await;
        assert!(matches!(unbound, Err(Error::WindowNotBound)));
    });
}
