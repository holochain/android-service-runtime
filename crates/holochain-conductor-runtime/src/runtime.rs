use crate::{RuntimeConfig, RuntimeError, RuntimeResult};
use holochain::{
    conductor::{
        api::{AdminInterfaceApi, AdminRequest, AdminResponse, AppInfo, ZomeCall},
        ConductorBuilder, ConductorHandle,
    },
    prelude::{InstallAppPayload, InstalledAppId, Signature, ZomeCallUnsigned},
};
use kitsune_p2p_types::dependencies::lair_keystore_api::dependencies::sodoken::BufRead;
use holochain::conductor::api::AppAuthenticationTokenIssued;
use holochain::conductor::api::IssueAppAuthenticationTokenPayload;
use holochain_types::websocket::AllowedOrigins;
use std::collections::HashMap;
use std::sync::{RwLock, Arc};

#[derive(Clone, Debug)]
pub struct AppWebsocket {
    pub authentication: AppAuthenticationTokenIssued,
    pub port: u16,
}

pub type AppWebsockets = Arc<RwLock<HashMap<InstalledAppId, AppWebsocket>>>;

/// Slim wrapper around holochain Conductor with calls wrapping AdminInterfaceApi requests
pub struct Runtime {
    conductor: ConductorHandle,
    app_websockets: AppWebsockets,
}

impl Runtime {
    /// Initialize and start a new Conductor
    pub async fn new(
        passphrase: Option<BufRead>,
        runtime_config: RuntimeConfig,
    ) -> RuntimeResult<Self> {
        let conductor = ConductorBuilder::default()
            .passphrase(passphrase)
            .config(runtime_config.clone().into())
            .build()
            .await?;

        Ok(Self { conductor, app_websockets: Arc::new(RwLock::new(HashMap::new())) })
    }

    /// Stop the Conductor
    ///
    /// This is *NOT* fully implemented by holochain: kitsune tasks will continue to run.
    pub async fn stop(&self) -> RuntimeResult<()> {
        self.conductor
            .shutdown()
            .await
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?
            .map_err(|e| RuntimeError::ConductorShutdown(e.to_string()))?;

        Ok(())
    }

    pub async fn install_app(&self, payload: InstallAppPayload) -> RuntimeResult<AppInfo> {
        let response = self
            .req_admin_api(AdminRequest::InstallApp(Box::new(payload)))
            .await?;
        match response {
            AdminResponse::AppInstalled(app_info) => Ok(app_info),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn uninstall_app(&self, installed_app_id: String) -> RuntimeResult<()> {
        let response = self
            .req_admin_api(AdminRequest::UninstallApp {
                installed_app_id,
                force: false,
            })
            .await?;
        match response {
            AdminResponse::AppUninstalled => Ok(()),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn enable_app(&self, installed_app_id: InstalledAppId) -> RuntimeResult<AppInfo> {
        let response = self
            .req_admin_api(AdminRequest::EnableApp { installed_app_id })
            .await?;
        match response {
            AdminResponse::AppEnabled { app, errors } => {
                if errors.is_empty() {
                    Ok(app)
                } else {
                    Err(RuntimeError::AdminApiAppEnabled(errors))
                }
            }
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn disable_app(&self, installed_app_id: InstalledAppId) -> RuntimeResult<()> {
        let response = self
            .req_admin_api(AdminRequest::EnableApp { installed_app_id })
            .await?;
        match response {
            AdminResponse::AppDisabled => Ok(()),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn list_apps(&self) -> RuntimeResult<Vec<AppInfo>> {
        let response = self
            .req_admin_api(AdminRequest::ListApps {
                status_filter: None,
            })
            .await?;
        match response {
            AdminResponse::AppsListed(apps) => Ok(apps),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    pub async fn is_app_installed(&self, installed_app_id: InstalledAppId) -> RuntimeResult<bool> {
        Ok(self
            .list_apps()
            .await?
            .into_iter()
            .any(|app_info| app_info.installed_app_id == installed_app_id))
    }

    pub async fn sign_zome_call(
        &self,
        zome_call_unsigned: ZomeCallUnsigned,
    ) -> RuntimeResult<ZomeCall> {
        // This is infallible, so unwrap is acceptable
        let pub_key: [u8; 32] = zome_call_unsigned.provenance.get_raw_32().try_into().unwrap();
        
        let signature_bytes = self
            .conductor
            .keystore()
            .lair_client()
            .sign_by_pub_key(pub_key.into(), None, zome_call_unsigned.data_to_sign()?)
            .await?;
        let signature = Signature::from(*signature_bytes.0);

        Ok(ZomeCall {
            cell_id: zome_call_unsigned.cell_id,
            zome_name: zome_call_unsigned.zome_name,
            fn_name: zome_call_unsigned.fn_name,
            payload: zome_call_unsigned.payload,
            cap_secret: zome_call_unsigned.cap_secret,
            provenance: zome_call_unsigned.provenance,
            nonce: zome_call_unsigned.nonce,
            expires_at: zome_call_unsigned.expires_at,
            signature,
        })
    }

    pub async fn ensure_app_websocket(&self, installed_app_id: InstalledAppId) -> RuntimeResult<AppWebsocket> {
        let app_websockets = self.app_websockets.read().unwrap().clone();
        match app_websockets.get(&installed_app_id) {
            Some(app_websocket) => Ok(app_websocket.clone()),
            None => {
                let authentication = self.issue_app_authentication_token(IssueAppAuthenticationTokenPayload {
                    installed_app_id: installed_app_id.clone(),
                    expiry_seconds: 0,
                    single_use: false,
                }).await?;
                let port = self.attach_app_interface(None, AllowedOrigins::Any, Some(installed_app_id.clone())).await?;
                let websocket = AppWebsocket {
                    authentication,
                    port
                };
                
                let mut app_websockets = self.app_websockets.write().unwrap();
                app_websockets.insert(installed_app_id, websocket.clone());

                Ok(websocket)
            }
        }
    }

    async fn req_admin_api(&self, request: AdminRequest) -> RuntimeResult<AdminResponse> {
        Ok(AdminInterfaceApi::new(self.conductor.clone())
            .handle_request(Ok(request))
            .await?)
    }

    async fn issue_app_authentication_token(&self, payload: IssueAppAuthenticationTokenPayload) -> RuntimeResult<AppAuthenticationTokenIssued> {
        let response = self
            .req_admin_api(AdminRequest::IssueAppAuthenticationToken(payload))
            .await?;
        match response {
            AdminResponse::AppAuthenticationTokenIssued(auth) => Ok(auth),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }

    async fn attach_app_interface(&self, port: Option<u16>, allowed_origins: AllowedOrigins, installed_app_id: Option<InstalledAppId>) -> RuntimeResult<u16> {
        let response = self
            .req_admin_api(AdminRequest::AttachAppInterface {
                port,
                allowed_origins,
                installed_app_id
            })
            .await?;
        match response {
            AdminResponse::AppInterfaceAttached { port } => Ok(port),
            fail => Err(RuntimeError::AdminApiBadResponse(fail)),
        }
    }
}
