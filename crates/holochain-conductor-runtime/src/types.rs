use holochain::conductor::api::{AppAuthenticationTokenIssued, AppInfo};

/// An app websocket port with an authentication token
#[derive(Clone, Debug)]
pub struct AppAuth {
    pub authentication: AppAuthenticationTokenIssued,
    pub port: u16,
}
