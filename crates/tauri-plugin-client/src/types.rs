use bytes::Bytes;
use holochain_conductor_runtime_types_ffi::RoleSettingsFfi;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetupAppConfig {
    pub app_id: String,
    pub happ_bundle_bytes: Bytes,
    pub network_seed: String,
    pub roles_settings: HashMap<String, RoleSettingsFfi>,
    pub enable_after_install: bool,
}
