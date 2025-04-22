/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'
import { type RoleSettings } from '@holochain/client';

export async function installApp(request: {
  installedAppId: string,
  source: Uint8Array,
  roleSettings: Map<String, RoleSettings>,
  networkSeed: String,
}): Promise<null> {
  return await invoke<null>('plugin:holochain-service-client|install_app', request);
}

export async function isAppInstalled(installedAppId: string): Promise<boolean> {
  return await invoke<{installed: boolean}>('plugin:holochain-service-client|is_app_installed', { installedAppId }).then((r) => (r.installed));
}

export async function ensureAppWebsocket(installedAppId: string): Promise<{port: number, authentication: {token: Uint8Array, expiresAt?: number}} | null> {
  return await invoke<{port: number, authentication: {token: Uint8Array, expiresAt?: number}}>('plugin:holochain-service-client|ensure_app_websocket', { installedAppId });
}