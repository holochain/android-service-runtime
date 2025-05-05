use crate::{RuntimeError, RuntimeResult};
use holochain::prelude::InstalledAppId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// A unique identifier representing the client instance
/// For example if the client is an android app, the client id would be the package name
/// i.e. "org.holochain.androidserviceclient.app"
#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct ClientId(pub String);

/// A map of ClientIds authorized to make app api requests to the specified InstalledAppIds
#[derive(Clone, Serialize, Deserialize, Debug)]
struct AuthorizedAppClients(HashMap<ClientId, Vec<InstalledAppId>>);

pub struct AuthorizedAppClientsManager {
    authorized_app_clients: Arc<RwLock<AuthorizedAppClients>>,
    persisted_path: PathBuf,
}

const PERSISTED_FILE_NAME: &str = "authorized_app_clients.msgpack";

impl AuthorizedAppClientsManager {
    pub fn new(data_root_path: PathBuf) -> Self {
        Self {
            authorized_app_clients: Arc::new(RwLock::new(AuthorizedAppClients(HashMap::new()))),
            persisted_path: data_root_path.join(PERSISTED_FILE_NAME),
        }
    }

    pub fn authorize(
        &self,
        client_uid: ClientId,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<()> {
        // Add the authorization to the in-memory data
        {
            let mut app_clients = self.authorized_app_clients.write().unwrap();
            let mut app_ids = match app_clients.0.clone().get(&client_uid) {
                Some(a) => a.clone(),
                None => vec![],
            };
            app_ids.push(installed_app_id);
            app_clients.0.insert(client_uid, app_ids.clone());
        }

        // Save all authorizations to the filesystem
        self.save_to_persisted()?;

        Ok(())
    }

    pub fn is_authorized(
        &self,
        client_uid: ClientId,
        installed_app_id: InstalledAppId,
    ) -> RuntimeResult<bool> {
        // Load all authorizations from the filesystem
        self.load_from_persisted()?;

        // Check the authorization from the in-memory data
        let app_clients = self.authorized_app_clients.read().unwrap();
        Ok(app_clients
            .0
            .get(&client_uid)
            .map(|app_ids| app_ids.contains(&installed_app_id))
            .unwrap_or_else(|| false))
    }

    fn save_to_persisted(&self) -> RuntimeResult<()> {
        // Encode in-memory value to bytes
        let authorized_app_clients = &self.authorized_app_clients.read().unwrap();
        let encoded = rmp_serde::to_vec(&authorized_app_clients.deref())
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileWriteError(e.to_string()))?;

        // Write bytes to file, creating file if it does not exist
        let mut file = File::create(self.persisted_path.clone())
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileWriteError(e.to_string()))?;

        file.write_all(encoded.as_slice())
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileWriteError(e.to_string()))?;

        Ok(())
    }

    fn load_from_persisted(&self) -> RuntimeResult<()> {
        // No-op if file does not exist
        let file_exists = std::fs::exists(self.persisted_path.clone())
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileReadError(e.to_string()))?;
        if !file_exists {
            return Ok(());
        }

        // Read bytes from file
        let mut f = File::open(self.persisted_path.clone())
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileReadError(e.to_string()))?;
        let mut encoded = vec![];
        f.read_to_end(&mut encoded)
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileReadError(e.to_string()))?;

        // Decode bytes
        let decoded: AuthorizedAppClients = rmp_serde::from_slice(encoded.as_slice())
            .map_err(|e| RuntimeError::AuthorizedAppClientsFileReadError(e.to_string()))?;

        // Update in-memory value
        let mut authorized_app_clients = self.authorized_app_clients.write().unwrap();
        *authorized_app_clients = decoded;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{AuthorizedAppClients, AuthorizedAppClientsManager, ClientId, PERSISTED_FILE_NAME};
    use std::collections::HashMap;
    use std::fs::{exists, File};
    use std::io::{Read, Write};
    use tempfile::tempdir;

    #[test]
    fn authorize_saves_to_file() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();
        let path = tmp_dir.path().join(PERSISTED_FILE_NAME);

        // Create new authorization manager and authorize an app pair
        let manager = AuthorizedAppClientsManager::new(tmp_dir.path().to_path_buf());
        manager
            .authorize(crate::ClientId("client-1".to_string()), "app-1".to_string())
            .unwrap();

        // Assert persisted file exists
        assert!(exists(manager.persisted_path.clone()).unwrap());

        // Read persisted file
        let mut f = File::open(path.clone()).unwrap();
        let mut encoded = vec![];
        f.read_to_end(&mut encoded).unwrap();
        let decoded: AuthorizedAppClients = rmp_serde::from_slice(encoded.as_slice()).unwrap();

        // Assert persisted file contains authorized app pair
        assert_eq!(
            decoded.0.get(&ClientId("client-1".to_string())),
            Some(&vec!["app-1".to_string()])
        );
    }

    #[test]
    fn is_authorized_loads_from_file() {
        // Create tempfile path
        let tmp_dir = tempdir().unwrap();
        let path = tmp_dir.path().join(PERSISTED_FILE_NAME);

        // Create new authorization manager using that file
        let manager = AuthorizedAppClientsManager::new(tmp_dir.path().to_path_buf());

        // Assert app pair is not authorized
        assert!(!manager
            .is_authorized(ClientId("client-1".to_string()), "app-1".to_string())
            .unwrap());

        // Write app pair to file
        let authorized_data = AuthorizedAppClients(HashMap::from([(
            ClientId("client-1".to_string()),
            vec!["app-1".to_string()],
        )]));
        let encoded = rmp_serde::to_vec(&authorized_data).unwrap();
        {
            let mut file = File::create(path.clone()).unwrap();
            file.write_all(encoded.as_slice()).unwrap();
        }

        // Assert app pair is authorized
        assert!(manager
            .is_authorized(ClientId("client-1".to_string()), "app-1".to_string())
            .unwrap())
    }
}
