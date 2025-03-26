package org.holochain.androidserviceruntime.holochain_service_client;

import org.holochain.androidserviceruntime.holochain_service_client.AppInfoFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.AppAuthFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallFfiParcel;

interface IHolochainServiceCallback {
    void listApps(in List<AppInfoFfiParcel> response);
    void installApp(in AppInfoFfiParcel response);
    void uninstallApp();
    void enableApp(in AppInfoFfiParcel response);
    void disableApp();
    void isAppInstalled(boolean response);
    void ensureAppWebsocket(in AppAuthFfiParcel response);
    void signZomeCall(in ZomeCallFfiParcel response);
}
