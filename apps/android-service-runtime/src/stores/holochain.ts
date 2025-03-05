import { writable, get, derived } from 'svelte/store';
import { launch, shutdown, enableApp, disableApp, uninstallApp, listInstalledApps, type AppInfo } from "tauri-plugin-holochain-service-api";
import { sortBy } from 'lodash';
import { addToast } from './toasts';
import { tick } from 'svelte';

function repaint() {
  return new Promise<void>(resolve =>
      requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
  );
}

export const isRunning = writable<boolean>(false);
export const installedApps = writable<AppInfo[]>([]);
export const loadingLoadInstalledApps = writable<boolean>(false);
export const loadingToggleEnableApp = writable<{ [key: string]: boolean}>({});
export const loadingLaunch = writable<boolean>(false);
export const loadingUninstallApp = writable<{[key: string]: boolean}>({});

export const loadInstalledApps = async () => {
  loadingLoadInstalledApps.set(true);
  await loadInstalledAppsInner();
  loadingLoadInstalledApps.set(false);
};
const loadInstalledAppsInner = async () => {
  try {
    installedApps.set(sortBy(await listInstalledApps(), 'installedAppId'));
  } catch(e) {
    console.error("Error fetching installed apps", e);
    addToast(`Error fetching installed apps ${e.message}`, "error");
  }
};


export const loadUninstallApp = async (appId: string) => {
  loadingUninstallApp.update((t) => ({...t, [appId]: true}));
  await tick();
  await repaint();
  try {
    await uninstallApp(appId);
    await loadInstalledAppsInner();
    addToast(`Uninstalled app "${appId}"`, "success");
  } catch(e) {
    console.error(`Error uninstalling app`, e);
    addToast(`Error uninstalling app ${e.message}`, "error");
  }
  loadingUninstallApp.update((t) => { delete t[appId]; return t; });
}

export const toggleEnableApp = async (appInfo: AppInfo) => {
  loadingToggleEnableApp.update((t) => ({...t, [appInfo.installedAppId]: true}));
  try {
    if(appInfo.status.type === "Running") {
      await disableApp(appInfo.installedAppId);
    } else {
      await enableApp(appInfo.installedAppId)
    }
    await loadInstalledAppsInner()
  } catch(e) {
    console.error("Error enabling/disabling app", e);
    addToast(`Error enabling/disabling down app ${e.message}`, "error");
  }
  loadingToggleEnableApp.update((t) => ({...t, [appInfo.installedAppId]: undefined}));
};

export const toggleLaunch = async () => {
  loadingLaunch.set(true);
  await tick();
  await repaint();
  
  try {
    if(!get(isRunning)) {
      await launch();
      await loadInstalledApps();
      isRunning.set(true);
    } else {
      await shutdown();
      isRunning.set(false);
    }
  } catch(e) {
    console.error("Error launching/shutting down conductor", e);
    addToast(`Error launching/shutting down conductor ${e.message}`, "error");
  }
  loadingLaunch.set(false);
}
