use holochain::conductor::config::{ConductorConfig, KeystoreConfig};
use kitsune_p2p_types::config::{KitsuneP2pConfig, TransportConfig};
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
            bootstrap_url: Url2::parse("https://bootstrap-0.infra.holochain.org"),
            signal_url: Url2::parse("wss://sbd.holo.host"),
            ice_urls: vec![Url2::parse("stun:stun.l.google.com:19302")],
        }
    }
}

impl From<RuntimeNetworkConfig> for KitsuneP2pConfig {
    fn from(val: RuntimeNetworkConfig) -> KitsuneP2pConfig {
        let mut res = KitsuneP2pConfig::default();

        let ice_urls: Vec<serde_json::Value> = val
            .ice_urls
            .clone()
            .into_iter()
            .map(|u| json!(u.to_string()))
            .collect();
        res.transport_pool.push(TransportConfig::WebRTC {
            signal_url: val.signal_url.clone().into(),
            webrtc_config: Some(json!({"ice_servers": {"urls": ice_urls}})),
        });
        res.bootstrap_service = Some(val.bootstrap_url.clone());

        res
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
