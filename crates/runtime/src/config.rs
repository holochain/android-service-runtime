use holochain::conductor::config::{ConductorConfig, KeystoreConfig};
use kitsune_p2p_types::config::{KitsuneP2pConfig, TransportConfig};
use serde_json::json;
use std::path::PathBuf;
use url2::Url2;

pub const DEVICE_SEED_LAIR_TAG: &str = "device-seed";

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    /// Path where conductor data is stored
    pub data_root_path: PathBuf,

    /// URL of the bootstrap server
    pub bootstrap_url: Url2,

    /// URL of the sbd server
    pub signal_url: Url2,

    /// URLs of ICE servers
    pub ice_urls: Vec<Url2>,
}

impl From<RuntimeConfig> for ConductorConfig {
    fn from(val: RuntimeConfig) -> Self {
        let mut conductor_config = ConductorConfig::default();
        let mut kitsune_config = KitsuneP2pConfig::default();

        conductor_config.device_seed_lair_tag = Some(DEVICE_SEED_LAIR_TAG.to_string());
        conductor_config.data_root_path = Some(val.data_root_path.clone().into());
        conductor_config.keystore = KeystoreConfig::LairServerInProc { lair_root: None };
        kitsune_config.bootstrap_service = Some(val.bootstrap_url);

        let ice_urls: Vec<serde_json::Value> = val
            .ice_urls
            .into_iter()
            .map(|u| json!(u.to_string()))
            .collect();
        kitsune_config.transport_pool.push(TransportConfig::WebRTC {
            signal_url: val.signal_url.into(),
            webrtc_config: Some(json!({"ice_servers": {"urls": ice_urls}})),
        });
        conductor_config.network = kitsune_config;

        conductor_config
    }
}
