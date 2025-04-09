/// Inject Holochain Client magic variables into window
/// Intended to be used by the HolochainPlugin.kt after getting an app websocket.

import { encode } from '@msgpack/msgpack';
import { type CallZomeRequest, type CallZomeRequestSigned } from '@holochain/client';

function injectHolochainClientEnv(installedAppId: string, port: number, token: Uint8Array) {
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

        const zomeCallUnsigned = {
            provenance: request.provenance,
            cellIdDnaHash: request.cell_id[0],
            cellIdAgentPubKey: request.cell_id[1],
            zomeName: request.zome_name,
            fnName: request.fn_name,
            capSecret: null,
            payload,
            nonce,
            expiresAt,
        };
        const response = await (window as any).__TAURI_INTERNALS__.invoke("plugin:holochain-service-consumer|sign_zome_call", zomeCallUnsigned);
        const zomeCallSigned = {
            provenance: request.provenance,
            cell_id: request.cell_id,
            zome_name: request.zome_name,
            fn_name: request.fn_name,
            cap_secret: null,
            payload,
            nonce,
            expires_at: expiresAt,
            signature: Uint8Array.from(response.signature),
        } as CallZomeRequestSigned;

        return zomeCallSigned;
    }
  };
}

(window as any).injectHolochainClientEnv = injectHolochainClientEnv;
