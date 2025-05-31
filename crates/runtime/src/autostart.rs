use crate::{Persisted, RuntimeNetworkConfig, RuntimeResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Name of file to persist data to
const PERSISTED_FILE_NAME: &str = "autostart_config.msgpack";

/// Is Auto Start on Boot enabled?
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct AutostartConfig {
    /// Is autostart enabled
    ///
    /// To enable autostart, set this to `Some(config)`,
    /// where `config` is the RuntimeNetworkConfig to apply when starting the runtime.
    ///
    /// To disable autostart, set this to `None`.
    enabled_config: Option<RuntimeNetworkConfig>,
}

/// Manager for reading & writing persisted autostart config
///
/// This does *not* actually handle starting the runtime,
/// but is purely a configuration, which the consumer of this
/// crate can read during its boot process.
#[derive(Clone)]
pub struct AutostartConfigManager {
    autostart_config: Arc<RwLock<AutostartConfig>>,
    persisted_path: PathBuf,
}

impl AutostartConfigManager {
    /// Initialize the AutostartConfigManager
    ///
    /// Read an existing config from the persisted state, if it exists.
    pub fn new(data_root_path: PathBuf) -> RuntimeResult<Self> {
        let persisted_path = data_root_path.join(PERSISTED_FILE_NAME);
        let data = Self::read_from_file(persisted_path.clone())?.unwrap_or_default();

        Ok(Self {
            autostart_config: Arc::new(RwLock::new(data)),
            persisted_path,
        })
    }

    /// Enable autostart with the specified network config
    pub fn enable(&self, config: RuntimeNetworkConfig) -> RuntimeResult<()> {
        // Add the config to the in-memory data
        {
            let mut autostart_config = self.autostart_config.write().unwrap();
            autostart_config.enabled_config = Some(config);
        }

        // Save all authorizations to the filesystem
        self.save_to_persisted()?;

        Ok(())
    }

    /// Disable autostart
    pub fn disable(&self) -> RuntimeResult<()> {
        // Add the config to the in-memory data
        {
            let mut autostart_config = self.autostart_config.write().unwrap();
            autostart_config.enabled_config = None;
        }

        // Save all authorizations to the filesystem
        self.save_to_persisted()?;

        Ok(())
    }

    /// Get the autostart config
    ///
    /// If autostart is enabled, returns `Ok(Some(config))`.
    /// If autostart is disabled, returns `Ok(None)`.
    pub fn get_enabled_config(&self) -> RuntimeResult<Option<RuntimeNetworkConfig>> {
        let autostart_config = self.autostart_config.read().unwrap();

        Ok(autostart_config.enabled_config.clone())
    }
}

impl Persisted<AutostartConfig> for AutostartConfigManager {
    fn get_file_path(&self) -> PathBuf {
        self.persisted_path.clone()
    }

    fn get_data_lock(&self) -> Arc<RwLock<AutostartConfig>> {
        self.autostart_config.clone()
    }
}

#[cfg(test)]
mod test {
    use super::{AutostartConfig, AutostartConfigManager, PERSISTED_FILE_NAME};
    use crate::RuntimeNetworkConfig;
    use std::fs::{exists, File};
    use std::io::{Read, Write};
    use tempfile::tempdir;
    use url2::Url2;

    #[test]
    fn authorize_saves_to_file() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();
        let path = tmp_dir.path().join(PERSISTED_FILE_NAME);

        // Create new autostart config and enable
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();
        let config = RuntimeNetworkConfig::default();
        manager.enable(config.clone()).unwrap();

        // Assert persisted file exists
        assert!(exists(manager.persisted_path.clone()).unwrap());

        // Read persisted file
        let mut f = File::open(path.clone()).unwrap();
        let mut encoded = vec![];
        f.read_to_end(&mut encoded).unwrap();
        let decoded: AutostartConfig = serde_json::from_slice(encoded.as_slice()).unwrap();

        // Assert persisted file contains authorized app pair
        assert_eq!(decoded.enabled_config.unwrap(), config);
    }

    #[test]
    fn autostart_loads_from_file() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();
        let path = tmp_dir.path().join(PERSISTED_FILE_NAME);

        // Create new autostart config manager using that file
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert app pair is not enabled
        assert!(manager.get_enabled_config().unwrap().is_none());

        // Write autostart config to file
        let autostart_config = AutostartConfig {
            enabled_config: Some(RuntimeNetworkConfig::default()),
        };
        let encoded = serde_json::to_string(&autostart_config).unwrap();
        {
            let mut file = File::create(path.clone()).unwrap();
            file.write_all(encoded.as_bytes()).unwrap();
        }

        // Create new autostart config manager using that file
        let manager2 = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert autostart is enabled
        assert!(manager2.get_enabled_config().unwrap().is_some())
    }

    #[test]
    fn enabled_persisted() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();

        // Create new autostart config manager using that file
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert app pair is not enabled
        assert!(manager.get_enabled_config().unwrap().is_none());

        // Enable
        let config = RuntimeNetworkConfig {
            bootstrap_url: Url2::parse("https://google.com"),
            ..RuntimeNetworkConfig::default()
        };
        let _ = manager.enable(config.clone());
        assert!(manager.get_enabled_config().unwrap().is_some());

        // Assert new instance is enabled with matching config
        let manager2 = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();
        assert_eq!(manager2.get_enabled_config().unwrap(), Some(config));
    }

    #[test]
    fn disabled_persisted() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();

        // Create new autostart config manager using that file
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert app pair is not enabled
        assert!(manager.get_enabled_config().unwrap().is_none());

        // Enable
        let _ = manager.enable(RuntimeNetworkConfig::default());
        assert!(manager.get_enabled_config().unwrap().is_some());

        // Disable
        let _ = manager.disable();
        assert!(manager.get_enabled_config().unwrap().is_none());

        // Assert new instance is disabled
        let manager2 = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();
        assert!(manager2.get_enabled_config().unwrap().is_none());
    }
}
