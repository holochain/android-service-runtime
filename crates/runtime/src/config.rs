use holochain::conductor::config::{ConductorConfig, KeystoreConfig, NetworkConfig};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use url2::Url2;

pub const DEVICE_SEED_LAIR_TAG: &str = "device-seed";

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    /// Path where conductor data is stored
    pub data_root_path: PathBuf,

    /// Network config
    pub network: RuntimeNetworkConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct RuntimeNetworkConfig {
    /// URL of the bootstrap server
    pub bootstrap_url: Url2,

    /// URL of the sbd server
    pub signal_url: Url2,

    /// URLs of ICE servers
    pub ice_urls: Vec<Url2>,
}

impl Default for RuntimeNetworkConfig {
    fn default() -> Self {
        Self {
            bootstrap_url: Url2::parse("https://relay.volla.tech"),
            signal_url: Url2::parse("wss://relay.volla.tech"),
            ice_urls: vec![Url2::parse("stun:stun.nextcloud.com:443")],
        }
    }
}

impl From<RuntimeNetworkConfig> for NetworkConfig {
    fn from(val: RuntimeNetworkConfig) -> NetworkConfig {
        NetworkConfig {
            bootstrap_url: val.bootstrap_url,
            signal_url: val.signal_url,
            webrtc_config: Some(json!({
                "iceServers": val.ice_urls
                    .clone()
                    .into_iter()
                    .map(|u| json!({"urls": [u]}))
                    .collect::<Vec<serde_json::Value>>()
            })),
            ..NetworkConfig::default()
        }
    }
}

impl From<RuntimeConfig> for ConductorConfig {
    fn from(val: RuntimeConfig) -> Self {
        Self {
            device_seed_lair_tag: Some(DEVICE_SEED_LAIR_TAG.to_string()),
            data_root_path: Some(val.data_root_path.clone().into()),
            keystore: KeystoreConfig::LairServerInProc { lair_root: None },
            network: val.network.into(),
            ..Self::default()
        }
    }
}
