use std::collections::HashMap;
use bytes::Bytes;
use serde::{Serialize, Deserialize};
use holochain_conductor_runtime_types_ffi::RoleSettingsFfi;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetupAppConfig {
    pub app_id: String,
    pub happ_bundle_bytes: Bytes,
    pub network_seed: String,
    pub role_settings: HashMap<String, RoleSettingsFfi>,
    pub enable_after_install: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SetupAppInvokeArg {
    pub source: Bytes,
    pub installed_app_id: String,
    pub network_seed: String,
    pub role_settings: HashMap<String, RoleSettingsFfi>,
    pub enable_after_install: bool,
}

impl From<SetupAppConfig> for SetupAppInvokeArg {
    fn from(val: SetupAppConfig) -> Self {
        Self {
            source: val.happ_bundle_bytes,
            installed_app_id: val.app_id,
            network_seed: val.network_seed,
            role_settings: val.role_settings,
            enable_after_install: val.enable_after_install
        }
    }
}
