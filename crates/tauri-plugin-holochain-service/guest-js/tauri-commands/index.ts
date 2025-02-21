/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'

export interface CellId {
  agentPubKey: Uint8Array;
  dnaHash: Uint8Array;
}

export interface Duration {
  secs: number;
  nanos: number;
}

export interface DnaModifiers {
  networkSeed: string;
  originTime: number;
  properties: Uint8Array;
  quantumTime: Duration;
}

export interface CellInfoV1 {
  cellId: CellId;
  dnaModifiers: DnaModifiers;
  name: string;
}

export interface CellInfo {
  v1: CellInfoV1;
}

export interface AppInfo {
  agentPubKey: Uint8Array;
  cellInfo: Map<string, CellInfo>;
}


export async function launch(): Promise<string | null> {
  return await invoke('plugin:holochain-service|launch');
}

export async function shutdown(): Promise<string | null> {
  return await invoke('plugin:holochain-service|shutdown');
}

export async function getAdminPort(): Promise<number | null> {
  return await invoke<{port?: number}>('plugin:holochain-service|get_admin_port').then((r) => (r.port ? r.port : null));
}

export async function installApp(request: {
  appId: string,
  appBundleBytes: Uint8Array,
  membraneProofs: Map<String, Uint8Array>,
  agent?: Uint8Array,
  networkSeed: String,
}): Promise<string | null> {
  return await invoke('plugin:holochain-service|install_app', request);
}

export async function uninstallApp(appId: string): Promise<null> {
  return await invoke('plugin:holochain-service|uninstall_app', { appId });
}

export async function enableApp(appId: string): Promise<null> {
  return await invoke('plugin:holochain-service|enable_app', { appId });
}

export async function disableApp(appId: string): Promise<null> {
  return await invoke('plugin:holochain-service|disable_app', { appId });
}

export async function listInstalledApps(): Promise<AppInfo[]> {
  return await invoke<{installedApps: AppInfo[]}>('plugin:holochain-service|list_installed_apps').then((r) => (r.installedApps ? r.installedApps : []));
}

export async function isAppInstalled(appId: string): Promise<boolean> {
  return await invoke<{installed: boolean}>('plugin:holochain-service|is_app_installed').then((r) => (r.installed));
}

export async function appWebsocketAuth(appId: string): Promise<{appId: string, port: number, token: Uint8Array} | null> {
  return await invoke<{appWebsocketAuth: {appId: string, port: number, token: Uint8Array}}>('plugin:holochain-service|app_websocket_auth', { appId }).then((r) => (r.appWebsocketAuth ? r.appWebsocketAuth : null));
}
