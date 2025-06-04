/// Helper functions to wrap tauri plugin commands
import { invoke } from '@tauri-apps/api/core';
export async function setConfig(request) {
    return await invoke('plugin:holochain-service|set_config', request);
}
export async function start() {
    return await invoke('plugin:holochain-service|start');
}
export async function stop() {
    return await invoke('plugin:holochain-service|stop');
}
export async function isReady() {
    return await invoke('plugin:holochain-service|is_ready').then((r) => (r.ready));
}
export async function installApp(request) {
    return await invoke('plugin:holochain-service|install_app', request);
}
export async function uninstallApp(installedAppId) {
    return await invoke('plugin:holochain-service|uninstall_app', { installedAppId });
}
export async function enableApp(installedAppId) {
    return await invoke('plugin:holochain-service|enable_app', { installedAppId }).then((r) => (r.enabled));
}
export async function disableApp(installedAppId) {
    return await invoke('plugin:holochain-service|disable_app', { installedAppId });
}
export async function listApps() {
    return await invoke('plugin:holochain-service|list_apps').then((r) => (r.installedApps ? r.installedApps : []));
}
export async function isAppInstalled(installedAppId) {
    return await invoke('plugin:holochain-service|is_app_installed', { installedAppId }).then((r) => (r.installed));
}
export async function ensureAppWebsocket(installedAppId) {
    return await invoke('plugin:holochain-service|ensure_app_websocket', { installedAppId });
}
