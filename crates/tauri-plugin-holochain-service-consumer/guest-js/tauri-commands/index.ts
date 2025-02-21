/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'

export async function installApp(request: {
  appId: string,
  appBundleBytes: Uint8Array,
  membraneProofs: Map<String, Uint8Array>,
  agent?: Uint8Array,
  networkSeed: String,
}): Promise<null> {
  return await invoke<null>('plugin:holochain-service-consumer|install_app', request);
}

export async function isAppInstalled(appId: string): Promise<boolean> {
  return await invoke<{installed: boolean}>('plugin:holochain-service-consumer|is_app_installed', { appId }).then((r) => (r.installed));
}

export async function appWebsocketAuth(appId: string): Promise<{appId: string, port: number, token: Uint8Array} | null> {
  return await invoke<{appId: string, port: number, token: Uint8Array}>('plugin:holochain-service-consumer|app_websocket_auth', { appId });
}