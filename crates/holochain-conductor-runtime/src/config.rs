use holochain::conductor::config::{ConductorConfig, KeystoreConfig};
use kitsune_p2p_types::config::{KitsuneP2pConfig, TransportConfig};
use std::path::PathBuf;
use url2::Url2;

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    /// Path where conductor data is stored
    pub data_root_path: PathBuf,

    /// URL of the bootstrap server
    pub bootstrap_url: Url2,

    /// URL of the sbd server
    pub signal_url: Url2,
}

impl From<RuntimeConfig> for ConductorConfig {
    fn from(val: RuntimeConfig) -> Self {
        let mut conductor_config = ConductorConfig::default();
        let mut kitsune_config = KitsuneP2pConfig::default();

        conductor_config.data_root_path = Some(val.data_root_path.clone().into());
        conductor_config.keystore = KeystoreConfig::LairServerInProc {
            lair_root: Some(
                PathBuf::from_iter([val.data_root_path, "lair-keystore".into()]).into(),
            ),
        };
        kitsune_config.bootstrap_service = Some(val.bootstrap_url);
        kitsune_config.transport_pool.push(TransportConfig::WebRTC {
            signal_url: val.signal_url.into(),
            webrtc_config: None,
        });
        conductor_config.network = kitsune_config;

        conductor_config
    }
}
