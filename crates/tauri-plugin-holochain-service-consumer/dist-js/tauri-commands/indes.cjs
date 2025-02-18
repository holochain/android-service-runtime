"use strict";
/// Helper functions to wrap tauri plugin commands
Object.defineProperty(exports, "__esModule", { value: true });
exports.appWebsocketAuth = exports.isAppInstalled = exports.installApp = void 0;
const core_1 = require("@tauri-apps/api/core");
async function installApp(request) {
    return await (0, core_1.invoke)('plugin:holochain-service-consumer|install_app', request);
}
exports.installApp = installApp;
async function isAppInstalled(appId) {
    return await (0, core_1.invoke)('plugin:holochain-service-consumer|is_app_installed', { appId }).then((r) => (r.installed));
}
exports.isAppInstalled = isAppInstalled;
async function appWebsocketAuth(appId) {
    return await (0, core_1.invoke)('plugin:holochain-service-consumer|app_websocket_auth', { appId });
}
exports.appWebsocketAuth = appWebsocketAuth;
