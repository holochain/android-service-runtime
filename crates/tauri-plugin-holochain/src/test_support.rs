//! Mock-app helpers for this crate's unit tests and `tests/integration.rs`, which
//! links the crate externally and so cannot see `#[cfg(test)]` items in `src/`.

use crate::{HolochainExt, HolochainPluginConfig, NetworkConfig};
use std::path::Path;
use std::time::Duration;
use tauri::test::{mock_builder, mock_context, noop_assets, MockRuntime};
use tauri::App;

const BOOT_TIMEOUT: Duration = Duration::from_secs(60);
const POLL_INTERVAL: Duration = Duration::from_millis(200);

pub fn plugin_config(data_dir: &Path) -> HolochainPluginConfig {
    HolochainPluginConfig::new(data_dir.to_path_buf(), NetworkConfig::default())
}

pub fn build_app(data_dir: &Path) -> App<MockRuntime> {
    mock_builder()
        .plugin(crate::init(
            crate::vec_to_locked(vec![]),
            plugin_config(data_dir),
        ))
        .build(mock_context(noop_assets()))
        .expect("failed to build mock tauri app")
}

/// Gating on `holochain()` alone is not enough: the plugin handle is managed
/// before the conductor boots, so `init_deferred` consumers can call `start`.
/// The readiness signal is the runtime, not the handle.
pub async fn wait_for_ready<R: tauri::Runtime>(app: &App<R>) {
    let ready = async {
        while app.holochain().and_then(|p| p.try_runtime()).is_err() {
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    };

    tokio::time::timeout(BOOT_TIMEOUT, ready)
        .await
        .unwrap_or_else(|_| panic!("conductor did not become ready within {BOOT_TIMEOUT:?}"));
}
