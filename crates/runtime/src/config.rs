use holochain::conductor::config::{ConductorConfig, KeystoreConfig, NetworkConfig};
use serde::{Deserialize, Serialize};
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

    /// URL of the iroh relay server
    pub relay_url: Url2,
}

impl Default for RuntimeNetworkConfig {
    fn default() -> Self {
        Self {
            bootstrap_url: Url2::parse("https://dev-test-bootstrap2.holochain.org"),
            relay_url: Url2::parse("https://use1-1.relay.n0.iroh-canary.iroh.link./"),
        }
    }
}

impl From<RuntimeNetworkConfig> for NetworkConfig {
    fn from(val: RuntimeNetworkConfig) -> NetworkConfig {
        NetworkConfig {
            bootstrap_url: val.bootstrap_url,
            relay_url: val.relay_url,
            ..NetworkConfig::default()
        }
    }
}

impl From<RuntimeConfig> for ConductorConfig {
    fn from(val: RuntimeConfig) -> Self {
        Self {
            data_root_path: Some(val.data_root_path.clone().into()),
            keystore: KeystoreConfig::LairServerInProc { lair_root: None },
            network: val.network.into(),
            ..Self::default()
        }
    }
}
