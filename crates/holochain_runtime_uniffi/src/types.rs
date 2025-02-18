use std::{collections::HashMap, time::Duration};

use holochain_conductor_api::{AppInfo, AppInfoStatus, CellInfo, ProvisionedCell, StemCell, ZomeCall};
use holochain_types::{app::{DisabledAppReason, PausedAppReason}, dna::{hash_type::{Agent, Dna}, HoloHash}, prelude::{CapSecret, CellId, ClonedCell, DnaModifiers, ExternIO, FunctionName, Nonce256Bits, Timestamp, ZomeCallUnsigned, ZomeName}};
use holochain_runtime::AppWebsocketAuth;

#[derive(uniffi::Record)]
pub struct DurationFFI {
  pub secs: u64,
  pub nanos: u32,
}

impl From<Duration> for DurationFFI {
  fn from(value: Duration) -> Self {
    Self {
      secs: value.as_secs(),
      nanos: value.subsec_nanos(),
    }
  }
}

#[derive(uniffi::Record)]
pub struct DnaModifiersFFI {
  pub network_seed: String,
  pub properties: Vec<u8>,
  pub origin_time: i64,
  pub quantum_time: DurationFFI,
}

impl From<DnaModifiers> for DnaModifiersFFI {
  fn from(value: DnaModifiers) -> Self {
    Self {
      network_seed: value.network_seed,
      properties: value.properties.bytes().to_owned(),
      origin_time: value.origin_time.0,
      quantum_time: value.quantum_time.into()
    }
  }
}

#[derive(uniffi::Record)]
pub struct ProvisionedCellFFI {
  pub cell_id: CellIdFFI,
  pub dna_modifiers: DnaModifiersFFI,
  pub name: String,
}

impl From<ProvisionedCell> for ProvisionedCellFFI {
  fn from(value: ProvisionedCell) -> Self {
    Self {
      cell_id: value.cell_id.into(),
      dna_modifiers: value.dna_modifiers.into(),
      name: value.name,
    }
  }
}

#[derive(uniffi::Record)]
pub struct ClonedCellFFI {
  pub cell_id: CellIdFFI,
  pub clone_id: String,
  pub original_dna_hash: Vec<u8>,
  pub dna_modifiers: DnaModifiersFFI,
  pub name: String,
  pub enabled: bool,
}

impl From<ClonedCell> for ClonedCellFFI {
  fn from(value: ClonedCell) -> Self {
    Self {
      cell_id: value.cell_id.into(),
      clone_id: value.clone_id.0,
      original_dna_hash: value.original_dna_hash.get_raw_39().to_vec(),
      dna_modifiers: value.dna_modifiers.into(),
      name: value.name,
      enabled: value.enabled,
    }
  }
}

#[derive(uniffi::Record)]
pub struct StemCellFFI {
  pub original_dna_hash: Vec<u8>,
  pub dna_modifiers: DnaModifiersFFI,
  pub name: Option<String>,
}

impl From<StemCell> for StemCellFFI {
  fn from(value: StemCell) -> Self {
    Self {
      original_dna_hash: value.original_dna_hash.get_raw_39().to_vec(),
      dna_modifiers: value.dna_modifiers.into(),
      name: value.name,
    }
  }
}

#[derive(uniffi::Enum)]
pub enum CellInfoFFI {
  Provisioned(ProvisionedCellFFI),
  Cloned(ClonedCellFFI),
  Stem(StemCellFFI),
}

impl From<CellInfo> for CellInfoFFI {
  fn from(value: CellInfo) -> Self {
    match value {
      CellInfo::Provisioned(provisioned) => CellInfoFFI::Provisioned(provisioned.into()),
      CellInfo::Cloned(cloned) => CellInfoFFI::Cloned(cloned.into()),
      CellInfo::Stem(stem) => CellInfoFFI::Stem(stem.into()),
    }
  }
}

#[derive(uniffi::Enum)]
pub enum PausedAppReasonFFI {
  Error(String)
}

impl From<PausedAppReason> for PausedAppReasonFFI {
  fn from(value: PausedAppReason) -> Self {
    match value {
      PausedAppReason::Error(error) => PausedAppReasonFFI::Error(error),
    }
  }
}

#[derive(uniffi::Enum)]
pub enum DisabledAppReasonFFI {
  NeverStarted,
  NotStartedAfterProvidingMemproofs,
  DeletingAgentKey,
  User,
  Error(String),  
}

impl From<DisabledAppReason> for DisabledAppReasonFFI {
  fn from(value: DisabledAppReason) -> Self {
    match value {
      DisabledAppReason::NeverStarted => DisabledAppReasonFFI::NeverStarted,
      DisabledAppReason::NotStartedAfterProvidingMemproofs => DisabledAppReasonFFI::NotStartedAfterProvidingMemproofs,
      DisabledAppReason::DeletingAgentKey => DisabledAppReasonFFI::DeletingAgentKey,
      DisabledAppReason::User => DisabledAppReasonFFI::User,
      DisabledAppReason::Error(error) => DisabledAppReasonFFI::Error(error),
    }
  }
}

#[derive(uniffi::Enum)]
pub enum AppInfoStatusFFI {
  Paused { reason: PausedAppReasonFFI },
  Disabled { reason: DisabledAppReasonFFI },
  Running,
  AwaitingMemproofs,
}

impl From<AppInfoStatus> for AppInfoStatusFFI {
  fn from(value: AppInfoStatus) -> Self {
    match value {
      AppInfoStatus::Paused {reason: paused } => AppInfoStatusFFI::Paused { reason: paused.into() },
      AppInfoStatus::Disabled { reason: disabled } => AppInfoStatusFFI::Disabled { reason: disabled.into() },
      AppInfoStatus::Running => AppInfoStatusFFI::Running,
      AppInfoStatus::AwaitingMemproofs => AppInfoStatusFFI::AwaitingMemproofs,
    }
  }
}

#[derive(uniffi::Record)]
pub struct AppInfoFFI {
  /// The unique identifier for an installed app in this conductor
  pub installed_app_id: String,
  pub cell_info: HashMap<String, Vec<CellInfoFFI>>,
  pub status: AppInfoStatusFFI,
  pub agent_pub_key: Vec<u8>,
}

impl From<AppInfo> for AppInfoFFI {
  fn from(value: AppInfo) -> Self {
    let mut cell_info: HashMap<String, Vec<CellInfoFFI>> = HashMap::new();
    for entry in value.cell_info.into_iter() {
      let entry_cell_infos: Vec<CellInfoFFI> = entry.1.into_iter().map(|val| val.into()).collect();
      cell_info.insert(entry.0, entry_cell_infos);
    }

    Self {
      installed_app_id: value.installed_app_id,
      cell_info,
      status: value.status.into(),
      agent_pub_key: value.agent_pub_key.into_inner()
    }
  }
}

#[derive(uniffi::Record)]
pub struct AppWebsocketAuthFFI {
    pub app_id: String,
    pub port: u16,
    pub token: Vec<u8>,
}

impl From<AppWebsocketAuth> for AppWebsocketAuthFFI {
  fn from(value: AppWebsocketAuth) -> Self {
      Self {
        app_id: value.app_id,
        port: value.app_websocket_port,
        token: value.token,
      }
  }
}

#[derive(uniffi::Record)]
pub struct CellIdFFI {
  pub dna_hash: Vec<u8>,
  pub agent_pub_key: Vec<u8>,
}

impl From<CellId> for CellIdFFI {
  fn from(value: CellId) -> Self {
    Self {
      dna_hash: value.dna_hash().get_raw_39().to_vec(),
      agent_pub_key: value.agent_pubkey().get_raw_39().to_vec(),
    }
  }
}

impl Into<CellId> for CellIdFFI {
  fn into(self) -> CellId {
    CellId::new(
      HoloHash::<Dna>::from_raw_39(self.dna_hash).unwrap(), 
      HoloHash::<Agent>::from_raw_39(self.agent_pub_key).unwrap()
    )
  }
}

#[derive(uniffi::Record)]
pub struct ZomeCallUnsignedFFI {
  pub provenance: Vec<u8>,
  pub cell_id: CellIdFFI,
  pub zome_name: String,
  pub fn_name: String,
  pub cap_secret: Option<Vec<u8>>,
  pub payload: Vec<u8>,
  pub nonce: Vec<u8>,
  pub expires_at: i64,
}

impl From<ZomeCallUnsigned> for ZomeCallUnsignedFFI {
  fn from(value: ZomeCallUnsigned) -> Self {
      Self {
        provenance: value.provenance.get_raw_39().to_vec(),
        cell_id: value.cell_id.into(),
        zome_name: value.zome_name.0.to_string(),
        fn_name: value.fn_name.into(),
        cap_secret: value.cap_secret.map(|s| s.as_ref().to_vec()),
        payload: value.payload.into(),
        nonce: value.nonce.into_inner().to_vec(),
        expires_at: value.expires_at.0
      }
  }
}

impl Into<ZomeCallUnsigned> for ZomeCallUnsignedFFI {
  fn into(self) -> ZomeCallUnsigned {
    let nonce: [u8; 32] = self.nonce.as_slice().try_into().unwrap();
    let cap_secret: Option<[u8; 64]> = self.cap_secret.map(|s| s.as_slice().try_into().unwrap());

    ZomeCallUnsigned {
        provenance: HoloHash::<Agent>::from_raw_39(self.provenance).unwrap(),
        cell_id: self.cell_id.into(),
        zome_name: ZomeName::new(self.zome_name),
        fn_name: FunctionName::new(self.fn_name),
        cap_secret: cap_secret.map(|s| CapSecret::from(s)),
        payload: ExternIO::from(self.payload),
        nonce: Nonce256Bits::from(nonce),
        expires_at: Timestamp(self.expires_at),
      }
  }
}

#[derive(uniffi::Record)]
pub struct ZomeCallFFI {
    pub cell_id: CellIdFFI,
    pub zome_name: String,
    pub fn_name: String,
    pub payload: Vec<u8>,
    pub cap_secret: Option<Vec<u8>>,
    pub provenance: Vec<u8>,
    pub signature: Vec<u8>,
    pub nonce: Vec<u8>,
    pub expires_at: i64,
}

impl From<ZomeCall> for ZomeCallFFI {
  fn from(value: ZomeCall) -> Self {
      Self {
        cell_id: value.cell_id.into(),
        zome_name: value.zome_name.0.to_string(),
        fn_name: value.fn_name.into(),
        payload: value.payload.into(),
        cap_secret: value.cap_secret.map(|s| s.as_ref().to_vec()),
        provenance: value.provenance.get_raw_39().to_vec(),
        signature: value.signature.0.to_vec(),
        nonce: value.nonce.into_inner().to_vec(),
        expires_at: value.expires_at.0
      }
  }
}
