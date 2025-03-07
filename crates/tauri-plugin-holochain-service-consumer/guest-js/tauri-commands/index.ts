/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'
import { type RoleSettings } from '@holochain/client';

export async function installApp(request: {
  installedAppId: string,
  source: Uint8Array,
  roleSettings: Map<String, RoleSettings>,
  networkSeed: String,
}): Promise<null> {
  return await invoke<null>('plugin:holochain-service-consumer|install_app', request);
}

export async function isAppInstalled(installedAppId: string): Promise<boolean> {
  return await invoke<{installed: boolean}>('plugin:holochain-service-consumer|is_app_installed', { installedAppId }).then((r) => (r.installed));
}

export async function ensureAppWebsocket(installedAppId: string): Promise<{installedAppId: string, port: number, token: Uint8Array} | null> {
  return await invoke<{installedAppId: string, port: number, token: Uint8Array}>('plugin:holochain-service-consumer|ensure_app_websocket', { installedAppId });
}