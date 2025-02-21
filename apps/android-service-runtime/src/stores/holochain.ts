import { writable, get, derived } from 'svelte/store';
import { launch, shutdown, getAdminPort, installApp, enableApp, disableApp, uninstallApp, listInstalledApps, type AppInfo } from "tauri-plugin-holochain-service-api";
import { v7 as uuidv7 } from 'uuid';
import { sortBy } from 'lodash';
import { addToast } from './toasts';
import { tick } from 'svelte';

function repaint() {
  return new Promise<void>(resolve =>
      requestAnimationFrame(() => requestAnimationFrame(() => resolve()))
  );
}

export const adminPort = writable<number>();
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
    if(!get(adminPort)) {
      await launch();
      await loadAdminPort();
    } else {
      await shutdown();
      adminPort.set(undefined);
    }
  } catch(e) {
    console.error("Error launching/shuttingdown app", e);
    addToast(`Error launching/shutting down app ${e.message}`, "error");
  }
  loadingLaunch.set(false);
}


export const loadAdminPort = async () => {
  return new Promise<void>((resolve) => {
    let interval = setInterval(async () => {
      if(!get(adminPort)) {
        const port = await getAdminPort();
        if(port) {
          adminPort.set(port);
          resolve();
        }
      } else {
        clearInterval(interval);
        resolve();
      }
    }, 1000);
  });
};

adminPort.subscribe(($adminPort) => { 
  if($adminPort !== undefined) {
    loadInstalledApps();
  }
});

export const isRunning = derived(adminPort, ($adminPort) => $adminPort !== undefined);