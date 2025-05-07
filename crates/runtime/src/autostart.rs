use crate::{Persisted, RuntimeResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Name of file to persit data to
const PERSISTED_FILE_NAME: &str = "autostart_config.msgpack";

/// Is Auto Start on Boot enabled?
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct AutostartConfig {
    enabled: bool,
}


/// Manager for reading & writing persisted autostart config
#[derive(Clone)]
pub struct AutostartConfigManager {
    autostart_config: Arc<RwLock<AutostartConfig>>,
    persisted_path: PathBuf,
}

impl AutostartConfigManager {
    pub fn new(data_root_path: PathBuf) -> RuntimeResult<Self> {
        let persisted_path = data_root_path.join(PERSISTED_FILE_NAME);
        let data = Self::read_from_file(persisted_path.clone())?.unwrap_or_default();

        Ok(Self {
            autostart_config: Arc::new(RwLock::new(data)),
            persisted_path,
        })
    }

    pub fn enable(&self) -> RuntimeResult<()> {
        // Add the config to the in-memory data
        {
            let mut autostart_config = self.autostart_config.write().unwrap();
            autostart_config.enabled = true;
        }

        // Save all authorizations to the filesystem
        self.save_to_persisted()?;

        Ok(())
    }

    pub fn disable(&self) -> RuntimeResult<()> {
        // Add the config to the in-memory data
        {
            let mut autostart_config = self.autostart_config.write().unwrap();
            autostart_config.enabled = false;
        }

        // Save all authorizations to the filesystem
        self.save_to_persisted()?;

        Ok(())
    }

    pub fn is_enabled(&self) -> RuntimeResult<bool> {
        let autostart_config = self.autostart_config.read().unwrap();

        Ok(autostart_config.enabled)
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
    use std::fs::{exists, File};
    use std::io::{Read, Write};
    use tempfile::tempdir;

    #[test]
    fn authorize_saves_to_file() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();
        let path = tmp_dir.path().join(PERSISTED_FILE_NAME);

        // Create new autostart config and enable
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();
        manager.enable().unwrap();

        // Assert persisted file exists
        assert!(exists(manager.persisted_path.clone()).unwrap());

        // Read persisted file
        let mut f = File::open(path.clone()).unwrap();
        let mut encoded = vec![];
        f.read_to_end(&mut encoded).unwrap();
        let decoded: AutostartConfig = serde_json::from_slice(encoded.as_slice()).unwrap();

        // Assert persisted file contains authorized app pair
        assert!(decoded.enabled);
    }

    #[test]
    fn autostart_loads_from_file() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();
        let path = tmp_dir.path().join(PERSISTED_FILE_NAME);

        // Create new autostart config manager using that file
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert app pair is not enabled
        assert!(!manager.is_enabled().unwrap());

        // Write autostart config to file
        let autostart_config = AutostartConfig { enabled: true };
        let encoded = serde_json::to_string(&autostart_config).unwrap();
        {
            let mut file = File::create(path.clone()).unwrap();
            file.write_all(encoded.as_bytes()).unwrap();
        }

        // Create new autostart config manager using that file
        let manager2 = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert autostart is enabled
        assert!(manager2.is_enabled().unwrap())
    }
    
    #[test]
    fn enabled_persisted() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();

        // Create new autostart config manager using that file
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert app pair is not enabled
        assert!(!manager.is_enabled().unwrap());

        // Enable
        let _ = manager.enable();
        assert!(manager.is_enabled().unwrap());

        // Assert new instance is enabled
        let manager2 = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();
        assert!(manager2.is_enabled().unwrap());
    }
 
    #[test]
    fn disabled_persisted() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();

        // Create new autostart config manager using that file
        let manager = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();

        // Assert app pair is not enabled
        assert!(!manager.is_enabled().unwrap());

        // Enable
        let _ = manager.enable();
        assert!(manager.is_enabled().unwrap());

        // Disable
        let _ = manager.disable();
        assert!(!manager.is_enabled().unwrap());

        // Assert new instance is disabled
        let manager2 = AutostartConfigManager::new(tmp_dir.path().to_path_buf()).unwrap();
        assert!(!manager2.is_enabled().unwrap());
    }
}
