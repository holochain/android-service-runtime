/// Injects the Holochain launcher env + a zome-call signer into a webview opened
/// by `tauri-plugin-holochain`, so `@holochain/client` connects to the in-process
/// conductor's app websocket and signs zome calls via the plugin's keystore.

import { encode } from '@msgpack/msgpack';
import { type CallZomeRequest, type CallZomeRequestSigned } from '@holochain/client';

function injectHolochainClientEnv(installedAppId: string, port: number, token: Uint8Array) {
  (window as any).__HC_LAUNCHER_ENV__ = {
    INSTALLED_APP_ID: installedAppId,
    APP_INTERFACE_PORT: port,
    APP_INTERFACE_TOKEN: token,
  };

  (window as any).__HC_ZOME_CALL_SIGNER__ = {
    signZomeCall: async (request: CallZomeRequest): Promise<CallZomeRequestSigned> => {
      const nonce = Uint8Array.from(await crypto.getRandomValues(new Uint8Array(32)));
      const expiresAt = 1e3 * (Date.now() + 3e5);
      const payload = Array.from(encode(request.payload));

      const zomeCall = {
        provenance: Array.from(request.provenance),
        cellIdDnaHash: Array.from(request.cell_id[0]),
        cellIdAgentPubKey: Array.from(request.cell_id[1]),
        zomeName: request.zome_name,
        fnName: request.fn_name,
        capSecret: null,
        payload,
        nonce: Array.from(nonce),
        expiresAt,
      };

      // The Rust `sign_zome_call` command takes a single `request` argument.
      const response = await (window as any).__TAURI_INTERNALS__.invoke(
        'plugin:holochain|sign_zome_call',
        { request: zomeCall }
      );

      return {
        bytes: Uint8Array.from(response.bytes),
        signature: Uint8Array.from(response.signature),
      } as CallZomeRequestSigned;
    },
  };
}

(window as any).injectHolochainClientEnv = injectHolochainClientEnv;
