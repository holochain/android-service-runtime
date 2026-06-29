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

/// Wait until the conductor has booted — i.e. the plugin's runtime is populated
/// (it emits `holochain://ready` at that point). Gating on `holochain()` alone
/// is not enough: the plugin handle is managed before the conductor boots (so
/// `init_deferred` consumers can call `start`), so the readiness signal is the
/// runtime, not the handle.
async fn wait_for_ready<R: tauri::Runtime>(app: &tauri::App<R>) {
    let mut waited = Duration::ZERO;
    let step = Duration::from_millis(200);
    while app.holochain().and_then(|p| p.try_runtime()).is_err() {
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

/// `rebind_window` re-routes a window's `app_request` to a different app — and
/// unbinds it — in place, so a consumer can move one persistent window between
/// apps (and to/from an app-less dashboard) without recreating it.
#[test]
fn rebind_window_reroutes_app_request_in_place() {
    let tmp = TempDir::new().unwrap();
    let app = build_app(&tmp);

    tauri::async_runtime::block_on(async move {
        wait_for_ready(&app).await;
        let plugin = app.holochain().unwrap();
        let runtime = plugin.runtime();

        // Two enabled apps to rebind between.
        install_and_enable_forum(&runtime).await;
        const APP_ID_2: &str = "forum-2";
        runtime
            .install_app(InstallAppPayload {
                source: AppBundleSource::Bytes(HAPP_FIXTURE.to_vec().into()),
                agent_key: None,
                installed_app_id: Some(APP_ID_2.into()),
                network_seed: Some(Uuid::new_v4().to_string()),
                roles_settings: Some(HashMap::new()),
                ignore_genesis_failure: false,
            })
            .await
            .expect("install forum-2 failed");
        runtime
            .enable_app(APP_ID_2.into())
            .await
            .expect("enable forum-2 failed");

        // Bind to the first app, then rebind to the second — the route follows
        // the binding, with no recreate.
        plugin
            .rebind_window("main", Some(APP_ID.into()))
            .await
            .unwrap();
        let resp: AppResponse = decode(
            &plugin
                .app_request_bytes("main", encode(&AppRequest::AppInfo).unwrap())
                .await
                .expect("app_request AppInfo failed"),
        )
        .unwrap();
        assert!(
            matches!(&resp, AppResponse::AppInfo(Some(i)) if i.installed_app_id == APP_ID),
            "expected AppInfo for {APP_ID}, got {resp:?}"
        );

        plugin
            .rebind_window("main", Some(APP_ID_2.into()))
            .await
            .unwrap();
        let resp: AppResponse = decode(
            &plugin
                .app_request_bytes("main", encode(&AppRequest::AppInfo).unwrap())
                .await
                .expect("app_request AppInfo failed"),
        )
        .unwrap();
        assert!(
            matches!(&resp, AppResponse::AppInfo(Some(i)) if i.installed_app_id == APP_ID_2),
            "expected AppInfo for {APP_ID_2}, got {resp:?}"
        );

        // Unbind — the window can no longer reach any app.
        plugin.rebind_window("main", None).await.unwrap();
        let unbound = plugin
            .app_request_bytes("main", encode(&AppRequest::AppInfo).unwrap())
            .await;
        assert!(matches!(unbound, Err(Error::WindowNotBound)));
    });
}

/// A *failed* rebind must not change the window's routing. `init_deferred`
/// leaves the runtime `NotReady`, so `rebind_window`'s `spawn_signal_forwarder`
/// fails deterministically — the previous binding must survive (no half-rebind
/// where `app_request` points at an app whose signal stream never started).
#[test]
fn rebind_failed_spawn_keeps_prior_binding() {
    let tmp = TempDir::new().unwrap();
    let app = mock_builder()
        .plugin(tauri_plugin_holochain::init_deferred(
            HolochainPluginConfig::new(tmp.path().to_path_buf(), NetworkConfig::default()),
        ))
        .build(mock_context(noop_assets()))
        .expect("failed to build mock tauri app");
    let plugin = app.holochain().unwrap();

    plugin.bind_window("main", APP_ID.into());
    assert_eq!(plugin.bound_app("main"), Some(APP_ID.to_string()));

    let result =
        tauri::async_runtime::block_on(plugin.rebind_window("main", Some("other-app".into())));
    assert!(
        matches!(result, Err(Error::NotReady)),
        "expected the rebind to fail with NotReady, got {result:?}"
    );
    assert_eq!(
        plugin.bound_app("main"),
        Some(APP_ID.to_string()),
        "a failed rebind must not flip the window's routing to the new app"
    );
}

/// Dropping a window clears its routing — the same `drop_window` path the
/// window-destroy handler (`plugin_builder`'s `on_event` → `RunEvent::WindowEvent`
/// `Destroyed`) runs to prune a closed window's maps and abort its forwarder. The
/// OS-window-destroy event itself only fires on a real window close, which the
/// mock test runtime doesn't drive, so this exercises the cleanup it calls.
#[test]
fn unbind_drops_window_routing() {
    let tmp = TempDir::new().unwrap();
    let app = mock_builder()
        .plugin(tauri_plugin_holochain::init_deferred(
            HolochainPluginConfig::new(tmp.path().to_path_buf(), NetworkConfig::default()),
        ))
        .build(mock_context(noop_assets()))
        .expect("failed to build mock tauri app");
    let plugin = app.holochain().unwrap();

    plugin.bind_window("main", APP_ID.into());
    assert_eq!(plugin.bound_app("main"), Some(APP_ID.to_string()));

    tauri::async_runtime::block_on(plugin.rebind_window("main", None)).unwrap();
    assert_eq!(
        plugin.bound_app("main"),
        None,
        "dropping a window must clear its routing"
    );
}
