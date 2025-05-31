/// Helper functions to wrap tauri plugin commands
import { invoke } from '@tauri-apps/api/core';
export async function installApp(request) {
    return await invoke('plugin:holochain-service-client|install_app', request);
}
export async function isAppInstalled(installedAppId) {
    return await invoke('plugin:holochain-service-client|is_app_installed', { installedAppId }).then((r) => (r.installed));
}
export async function ensureAppWebsocket(installedAppId) {
    return await invoke('plugin:holochain-service-client|ensure_app_websocket', { installedAppId });
}
