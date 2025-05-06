use crate::{RuntimeError, RuntimeResult};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

pub trait Persisted<D: Serialize + for<'a> Deserialize<'a>> {
    /// Read data from file and decode as msgpack
    fn read_from_file(path: PathBuf) -> RuntimeResult<Option<D>> {
        // No-op if file does not exist
        let file_exists = std::fs::exists(path.clone())
            .map_err(|e| RuntimeError::PersistedFileReadError(e.to_string()))?;
        if !file_exists {
            return Ok(None);
        }

        // Read bytes from file
        let mut f = File::open(path.clone())
            .map_err(|e| RuntimeError::PersistedFileReadError(e.to_string()))?;
        let mut encoded = vec![];
        f.read_to_end(&mut encoded)
            .map_err(|e| RuntimeError::PersistedFileReadError(e.to_string()))?;

        // Decode bytes
        let decoded: D = rmp_serde::from_slice(encoded.as_slice())
            .map_err(|e| RuntimeError::PersistedFileReadError(e.to_string()))?;

        Ok(Some(decoded))
    }

    /// Save in-memory data to persistence file
    fn save_to_persisted(&self) -> RuntimeResult<()> {
        // Encode in-memory value to bytes
        let lock = self.get_data_lock();
        let data = lock.read().unwrap();
        let encoded = rmp_serde::to_vec(&data.deref())
            .map_err(|e| RuntimeError::PersistedFileWriteError(e.to_string()))?;

        // Write bytes to file, creating file if it does not exist
        let mut file = File::create(self.get_file_path().clone())
            .map_err(|e| RuntimeError::PersistedFileWriteError(e.to_string()))?;

        file.write_all(encoded.as_slice())
            .map_err(|e| RuntimeError::PersistedFileWriteError(e.to_string()))?;

        Ok(())
    }

    /// Load in-memory data from persistence file
    /// No-op if file does not exist
    fn load_from_persisted(&self) -> RuntimeResult<()> {
        if let Some(decoded) = Self::read_from_file(self.get_file_path())? {
            // Update in-memory value
            let lock = self.get_data_lock();
            let mut data = lock.write().unwrap();
            *data = decoded;
        }

        Ok(())
    }
    
    /// Path to persistence file
    fn get_file_path(&self) -> PathBuf;

    /// In-memory copy of data
    fn get_data_lock(&self) -> Arc<RwLock<D>>;
}
