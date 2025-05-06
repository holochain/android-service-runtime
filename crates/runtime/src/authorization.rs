use crate::{Persisted, RuntimeResult};
use holochain::prelude::InstalledAppId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

const PERSISTED_FILE_NAME: &str = "authorized_app_clients.msgpack";

/// A unique identifier representing the client instance
/// For example if the client is an android app, the client id would be the package name
/// i.e. "org.holochain.androidserviceclient.app"
#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct ClientId(pub String);

/// A map of ClientIds authorized to make app api requests to the specified InstalledAppIds
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
struct AuthorizedAppClients(HashMap<ClientId, Vec<InstalledAppId>>);

pub struct AuthorizedAppClientsManager {
    authorized_app_clients: Arc<RwLock<AuthorizedAppClients>>,
    persisted_path: PathBuf,
}

impl AuthorizedAppClientsManager {
    pub fn new(data_root_path: PathBuf) -> RuntimeResult<Self> {
        let persisted_path = data_root_path.join(PERSISTED_FILE_NAME);
        let data = Self::read_from_file(persisted_path.clone())?.unwrap_or_default();

        Ok(Self {
            authorized_app_clients: Arc::new(RwLock::new(data)),
            persisted_path,
        })
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
}

impl Persisted<AuthorizedAppClients> for AuthorizedAppClientsManager {
    fn get_file_path(&self) -> PathBuf {
        self.persisted_path.join(PERSISTED_FILE_NAME)
    }

    fn get_data_lock(&self) -> Arc<RwLock<AuthorizedAppClients>> {
        self.authorized_app_clients.clone()
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
        let manager = AuthorizedAppClientsManager::new(tmp_dir.path().to_path_buf()).unwrap();
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
        let manager = AuthorizedAppClientsManager::new(tmp_dir.path().to_path_buf()).unwrap();

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
