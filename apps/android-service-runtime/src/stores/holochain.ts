import { writable, get } from 'svelte/store';
import { start, stop, isReady, enableApp, disableApp, uninstallApp, listApps, type AppInfo } from "tauri-plugin-holochain-service-api";
import { sortBy } from 'lodash';
import { addToast } from './toasts';
import { tick } from 'svelte';
import retry from 'async-retry';
import { RUNTIME_NETWORK_CONFIG } from '../config';

function repaint() {
  return new Promise<void>(resolve =>
      requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
  );
}

export const isRunning = writable<boolean>(false);
export const apps = writable<AppInfo[]>([]);
export const loadingApps = writable<boolean>(false);
export const loadingToggleEnableApp = writable<{ [key: string]: boolean}>({});
export const loadingLaunch = writable<boolean>(false);
export const loadingUninstallApp = writable<{[key: string]: boolean}>({});

export const loadApps = async () => {
  loadingApps.set(true);
  await loadAppsInner();
  loadingApps.set(false);
};
const loadAppsInner = async () => {
  try {
    apps.set(sortBy(await listApps(), 'installedAppId'));
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
    await loadAppsInner();
    addToast(`Uninstalled app "${appId}"`, "success");
  } catch(e) {
    console.error(`Error uninstalling app`, e);
    addToast(`Error uninstalling app ${e.message}`, "error");
  }
  loadingUninstallApp.update((t) => { delete t[appId]; return t; });
}

export const loadIsRunning = async () => {
  const ready = await isReady();
  isRunning.set(ready);
}

export const toggleEnableApp = async (appInfo: AppInfo) => {
  loadingToggleEnableApp.update((t) => ({...t, [appInfo.installedAppId]: true}));
  try {
    if(appInfo.status.type === "Running") {
      await disableApp(appInfo.installedAppId);
    } else {
      await enableApp(appInfo.installedAppId)
    }
    await loadAppsInner()
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
  
  if(!get(isRunning)) {
    await startInner();
    isRunning.set(true)
  } else {
    await stopInner();
    isRunning.set(false)
  }
  loadingLaunch.set(false);
}

const startInner = async () => {
  try {
    await start();
    await retry(async () => {
      const ready = await isReady();
      if(!ready) throw new Error("Conductor not ready");
    }, {
      retries: 500,
      factor: 1,
      minTimeout: 500
    });
  } catch(e) {
    console.error("Error starting conductor", e);
    addToast(`Error starting conductor ${e.message}`, "error");
  }
};

const stopInner = async () => {
  try {
    await stop();
  } catch(e) {
    console.error("Error stopping conductor", e);
    addToast(`Error stopping conductor ${e.message}`, "error");
  }
};
