use holochain::conductor::config::{ConductorConfig, KeystoreConfig};
use kitsune_p2p_types::config::{KitsuneP2pConfig, TransportConfig};
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

        let mut ice_urls = serde_json::Map::new();
        ice_urls.insert("urls".to_string(), serde_json::Value::Array(val.ice_urls.into_iter().map(|u| serde_json::Value::from(u.to_string())).collect()));
        let mut webrtc_config = serde_json::Map::new();
        webrtc_config.insert("ice_servers".to_string(), serde_json::Value::Object(ice_urls));

        kitsune_config.transport_pool.push(TransportConfig::WebRTC {
            signal_url: val.signal_url.into(),
            webrtc_config: Some(serde_json::Value::Object(webrtc_config)),
        });
        conductor_config.network = kitsune_config;

        conductor_config
    }
}
