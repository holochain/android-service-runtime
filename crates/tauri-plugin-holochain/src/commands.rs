use crate::{HolochainExt, Result};
use holochain_conductor_runtime_types_ffi::{CellIdFfi, ZomeCallParamsFfi};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};

/// Unsigned zome-call params as sent by the webview zome-call signer
/// (`__HC_ZOME_CALL_SIGNER__`). This is the flat, camelCase shape `@holochain/client`
/// produces — note `cell_id` is split into its two hashes here, matching the JS.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SignZomeCallRequest {
    provenance: Vec<u8>,
    cell_id_dna_hash: Vec<u8>,
    cell_id_agent_pub_key: Vec<u8>,
    zome_name: String,
    fn_name: String,
    cap_secret: Option<Vec<u8>>,
    payload: Vec<u8>,
    nonce: Vec<u8>,
    expires_at: i64,
}

/// Signed zome call returned to the webview signer.
#[derive(Serialize)]
pub(crate) struct SignZomeCallResponse {
    bytes: Vec<u8>,
    signature: Vec<u8>,
}

/// Sign a zome call with the conductor's keystore.
///
/// Invoked by the signer injected into the webview, so `@holochain/client` in
/// the UI can make authenticated zome calls without direct keystore access.
#[tauri::command]
pub(crate) async fn sign_zome_call<R: Runtime>(
    app: AppHandle<R>,
    request: SignZomeCallRequest,
) -> Result<SignZomeCallResponse> {
    // Reuse the runtime-types-ffi conversion to build holochain's ZomeCallParams.
    let params = ZomeCallParamsFfi {
        provenance: request.provenance,
        cell_id: CellIdFfi {
            dna_hash: request.cell_id_dna_hash,
            agent_pub_key: request.cell_id_agent_pub_key,
        },
        zome_name: request.zome_name,
        fn_name: request.fn_name,
        cap_secret: request.cap_secret,
        payload: request.payload,
        nonce: request.nonce,
        expires_at: request.expires_at,
    };

    let signed = app
        .holochain()?
        .runtime()
        .sign_zome_call(params.into())
        .await?;

    Ok(SignZomeCallResponse {
        bytes: signed.bytes.into(),
        signature: signed.signature.0.into(),
    })
}
