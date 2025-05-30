/// Helper functions to wrap tauri plugin commands

import { invoke } from '@tauri-apps/api/core'

export interface CellId {
  agentPubKey: Uint8Array;
  dnaHash: Uint8Array;
}

export interface DnaModifiers {
  networkSeed: string;
  properties: Uint8Array;
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

export type RuntimeNetworkConfig = {
  bootstrapUrl: string;
  signalUrl: string;
  iceUrls: string[];
}

export async function setConfig(request: RuntimeNetworkConfig): Promise<null> {
  return await invoke('plugin:holochain-service|set_config', request);
}

export async function start(): Promise<string | null> {
  return await invoke('plugin:holochain-service|start');
}

export async function stop(): Promise<string | null> {
  return await invoke('plugin:holochain-service|stop');
}

export async function isReady(): Promise<boolean> {
  return await invoke<{ready: boolean}>('plugin:holochain-service|is_ready').then((r) => (r.ready));
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

export async function uninstallApp(installedAppId: string): Promise<null> {
  return await invoke('plugin:holochain-service|uninstall_app', { installedAppId });
}

export async function enableApp(installedAppId: string): Promise<AppInfo> {
  return await invoke<{enabled: AppInfo}>('plugin:holochain-service|enable_app', { installedAppId }).then((r) => (r.enabled));
}

export async function disableApp(installedAppId: string): Promise<null> {
  return await invoke('plugin:holochain-service|disable_app', { installedAppId });
}

export async function listApps(): Promise<AppInfo[]> {
  return await invoke<{installedApps: AppInfo[]}>('plugin:holochain-service|list_apps').then((r) => (r.installedApps ? r.installedApps : []));
}

export async function isAppInstalled(installedAppId: string): Promise<boolean> {
  return await invoke<{installed: boolean}>('plugin:holochain-service|is_app_installed', { installedAppId }).then((r) => (r.installed));
}

export async function ensureAppWebsocket(installedAppId: string): Promise<{port: number, authentication: {token: Uint8Array, expiresAt?: number}} | null> {
  return await invoke<{port: number, authentication: {token: Uint8Array, expiresAt?: number}}>('plugin:holochain-service|ensure_app_websocket', { installedAppId });
}
