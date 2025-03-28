/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'
import { AppInfo, type RoleSettings } from '@holochain/client';

export async function connect(request: {
  installedAppId: string
}): Promise<null> {
  return await invoke<null>('plugin:holochain-service-consumer|connect', request);
}

export async function installApp(request: {
  source: Uint8Array,
  roleSettings: Map<String, RoleSettings>,
  networkSeed: String,
}): Promise<null> {
  return await invoke<null>('plugin:holochain-service-consumer|install_app', request);
}

export async function enableApp(): Promise<AppInfo> {
  return await invoke<{enabled: AppInfo}>('plugin:holochain-service-consumer|enable_app').then((r) => (r.enabled));
}

export async function isAppInstalled(): Promise<boolean> {
  return await invoke<{installed: boolean}>('plugin:holochain-service-consumer|is_app_installed').then((r) => (r.installed));
}

export async function ensureAppWebsocket(): Promise<{installedAppId: string, port: number, token: Uint8Array} | null> {
  return await invoke<{installedAppId: string, port: number, token: Uint8Array}>('plugin:holochain-service-consumer|ensure_app_websocket');
}