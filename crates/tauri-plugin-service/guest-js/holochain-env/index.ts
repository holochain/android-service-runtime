/// Inject Holochain Client magic variables into window
/// Intended to be used by the HolochainPlugin.kt after getting an app websocket.

import { encode } from '@msgpack/msgpack';
import { type CallZomeRequest, type CallZomeRequestSigned } from '@holochain/client';

function injectHolochainClientEnv(installedAppId: String, port: number, token: Uint8Array) {
  (window as any).__HC_LAUNCHER_ENV__ = {
    INSTALLED_APP_ID: installedAppId,
    APP_INTERFACE_PORT: port,
    APP_INTERFACE_TOKEN: token
  };

  (window as any).__HC_ZOME_CALL_SIGNER__ = {
    signZomeCall: async (request: CallZomeRequest): Promise<CallZomeRequestSigned> => {
        const nonce = Uint8Array.from(await crypto.getRandomValues(new Uint8Array(32)));
        const expiresAt = 1e3*(Date.now()+3e5);
        const payload = Array.from(encode(request.payload));

        const ZomeCallParams = {
            provenance: request.provenance,
            cellId: request.cell_id,
            zomeName: request.zome_name,
            fnName: request.fn_name,
            capSecret: null,
            payload,
            nonce,
            expiresAt,
        };
        const response = await (window as any).__TAURI_INTERNALS__.invoke("plugin:holochain-service|sign_zome_call", ZomeCallParams);
        const zomeCallSigned = {
            bytes: Uint8Array.from(response.bytes),
            signature: Uint8Array.from(response.signature),
        } as CallZomeRequestSigned;

        return zomeCallSigned;
    }
};
}

(window as any).injectHolochainClientEnv = injectHolochainClientEnv;