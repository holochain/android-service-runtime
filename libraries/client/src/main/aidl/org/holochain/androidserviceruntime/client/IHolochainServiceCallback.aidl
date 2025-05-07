package org.holochain.androidserviceruntime.client;

import org.holochain.androidserviceruntime.client.AppInfoFfiParcel;
import org.holochain.androidserviceruntime.client.AppAuthFfiParcel;
import org.holochain.androidserviceruntime.client.ZomeCallFfiParcel;
import org.holochain.androidserviceruntime.client.AdminBinderUnauthorizedExceptionParcel;
import org.holochain.androidserviceruntime.client.AppBinderUnauthorizedExceptionParcel;

interface IHolochainServiceCallback {
    // Success responses for each call
    void listApps(in List<AppInfoFfiParcel> response);
    void setupApp(in AppAuthFfiParcel response);
    void installApp(in AppInfoFfiParcel response);
    void uninstallApp();
    void enableApp(in AppInfoFfiParcel response);
    void disableApp();
    void isAppInstalled(boolean response);
    void ensureAppWebsocket(in AppAuthFfiParcel response);
    void signZomeCall(in ZomeCallFfiParcel response);

    // Error responses
    void adminBinderUnauthorizedException(in AdminBinderUnauthorizedExceptionParcel response);
    void appBinderUnauthorizedException(in AppBinderUnauthorizedExceptionParcel response);
}
