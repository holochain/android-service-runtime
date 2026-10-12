//! iOS: keep lair's domain socket path under the platform limit.
//!
//! Lair serves the conductor over a unix domain socket, by default `<data_root>/socket`. Socket paths are limited to
//! `SUN_LEN` (104 bytes on Apple platforms), and iOS app container paths are longer than that, so lair fails with
//! "path must be shorter than SUN_LEN". Lair only accepts absolute socket paths, so before the conductor starts we
//! write (or patch) its config with a short absolute socket path: the app's temp directory on devices
//! (`/private/var/mobile/Containers/Data/Application/<uuid>/tmp`, ~90 bytes) or `/tmp` on the simulator, whose app
//! paths are longer but which isn't sandboxed.

use lair_keystore_api::config::LairServerConfigInner;
use lair_keystore_api::types::SharedLockedArray;
use std::path::{Path, PathBuf};

/// Longest socket path we use; SUN_LEN on Apple is 104 including the terminating NUL.
const MAX_SOCKET_PATH: usize = 100;

pub(crate) async fn ensure_short_lair_socket(
    config_path: &Path,
    passphrase: SharedLockedArray,
) -> Result<(), String> {
    let lair_root = config_path.parent().ok_or("lair config has no parent directory")?;

    let mut config = match tokio::fs::read(config_path).await {
        Ok(bytes) => LairServerConfigInner::from_bytes(&bytes).map_err(|e| e.to_string())?,
        Err(_) => {
            tokio::fs::create_dir_all(lair_root).await.map_err(|e| e.to_string())?;
            LairServerConfigInner::new(lair_root, passphrase)
                .await
                .map_err(|e| e.to_string())?
        }
    };

    let current = config.get_connection_path();
    if current.as_os_str().len() <= MAX_SOCKET_PATH {
        return Ok(());
    }

    let socket = short_socket_path(lair_root)?;
    let key = config.get_server_pub_key().map_err(|e| e.to_string())?;
    config.connection_url = lair_keystore_api::dependencies::url::Url::parse(&format!("unix://{}?k={}", socket.display(), key))
        .map_err(|e| e.to_string())?;
    log::info!("lair socket moved to {} (the default path is too long for a domain socket)", socket.display());

    tokio::fs::write(config_path, config.to_string()).await.map_err(|e| e.to_string())
}

/// `<temp dir>/hc.sock` when short enough, else `/tmp/hc-<hash of the data dir>.sock` (one per app install).
fn short_socket_path(lair_root: &Path) -> Result<PathBuf, String> {
    let in_temp = std::env::temp_dir().join("hc.sock");
    if in_temp.as_os_str().len() <= MAX_SOCKET_PATH {
        return Ok(in_temp);
    }

    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    lair_root.hash(&mut hasher);
    let in_tmp = PathBuf::from(format!("/tmp/hc-{:016x}.sock", hasher.finish()));
    Ok(in_tmp)
}
