use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstallAppRequestArgs {
  pub app_id: String,
  pub app_bundle_bytes: Vec<u8>,
  pub membrane_proofs: HashMap<String, Vec<u8>>,
  pub agent: Option<Vec<u8>>,
  pub network_seed: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppIdRequestArgs {
  pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppWebsocketAuthResponse {
  pub app_id: String,
  pub port: u16,
  pub token: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignZomeCallRequestArgs {
  pub provenance: Vec<u8>,
  pub cell_id_dna_hash: Vec<u8>,
  pub cell_id_agent_pub_key: Vec<u8>,
  pub zome_name: String,
  pub fn_name: String,
  pub cap_secret: Option<Vec<u8>>,
  pub payload: Vec<u8>,
  pub nonce: Vec<u8>,
  pub expires_at: u64,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignZomeCallResponse {
  pub provenance: Vec<u8>,
  pub cell_id_dna_hash: Vec<u8>,
  pub cell_id_agent_pub_key: Vec<u8>,
  pub zome_name: String,
  pub fn_name: String,
  pub cap_secret: Option<Vec<u8>>,
  pub payload: Vec<u8>,
  pub nonce: Vec<u8>,
  pub expires_at: u64,
  pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IsAppInstalledResponse {
  pub installed: bool
}