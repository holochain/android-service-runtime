use crate::error::HolochainRuntimeFFIConfigError;
use holochain_runtime::{HolochainRuntimeConfig, WANNetworkConfig, GossipArcClamp};
use std::path::PathBuf;
use std::str::FromStr;
use url2::{Url2, Url2Error};


#[derive(uniffi::Enum)]
pub enum GossipArcClampFFI {
    Full,
    Empty
}

impl Into<GossipArcClamp> for GossipArcClampFFI {
    fn into(self) -> GossipArcClamp {
        match self {
            GossipArcClampFFI::Full => GossipArcClamp::Full,
            GossipArcClampFFI::Empty => GossipArcClamp::Empty,
        }
    }
}

#[derive(uniffi::Record)]
pub struct HolochainRuntimeFFIConfig {
    /// URL of bootstrap server
    bootstrap_url: String,

    /// URL of signal server
    signal_url: String,

    /// Path to directory where conductor data will be stored
    holochain_dir: String,

    /// List of ICE server URLs
    ice_servers_urls: Vec<String>,
    
    /// Force the conductor to always have a "full", or "empty" Gossip Arc for all DNAs.
    /// The Gossip Arc is the subsection of the DHT that you aim to store and serve to others.
    ///
    /// A Full Gossip Arc means that your peer will always try to hold the full DHT state,
    /// and serve it to others.
    ///
    /// An Empty Gossip Arc means that your peer will always go to the network to fetch DHT data,
    /// unless they authored it.
    gossip_arc_clamp: Option<GossipArcClampFFI>,

    /// Fallback to LAN only mode if the signal server configured in WANNetworkConfig can't be 
    /// reached at launch
    fallback_to_lan_only: bool
}

impl TryInto<HolochainRuntimeConfig> for HolochainRuntimeFFIConfig {
    type Error = HolochainRuntimeFFIConfigError;
    fn try_into(self) -> Result<HolochainRuntimeConfig, Self::Error> {
        Ok(HolochainRuntimeConfig {
            holochain_dir: PathBuf::from_str(self.holochain_dir.as_str())?,
            wan_network_config: Some(WANNetworkConfig {
                bootstrap_url: Url2::try_parse(self.bootstrap_url)?,
                signal_url: Url2::try_parse(self.signal_url)?,
                ice_servers_urls: self.ice_servers_urls.into_iter().map(|s| Url2::try_parse(s)).collect::<Result<Vec<Url2>, Url2Error>>()?,
            }),
            admin_port: None,
            gossip_arc_clamp: self.gossip_arc_clamp.map(|c| c.into()),
            fallback_to_lan_only: self.fallback_to_lan_only,
        })
    }
}
