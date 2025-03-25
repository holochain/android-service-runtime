package org.holochain.androidserviceruntime.holochain_service_client;

import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceCallback;
import org.holochain.androidserviceruntime.holochain_service_client.InstallAppPayloadFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.AppInfoFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.AppAuthFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallFfiParcel;

interface IHolochainService {
    void stop();
    boolean isReady();
    void installApp(IHolochainServiceCallback callback, in InstallAppPayloadFfiParcel request);
    void uninstallApp(IHolochainServiceCallback callback, String installedAppId);
    AppInfoFfiParcel enableApp(String installedAppId);
    void disableApp(String installedAppId);
    void listApps(IHolochainServiceCallback callback);
    boolean isAppInstalled(String installedAppId);
    AppAuthFfiParcel ensureAppWebsocket(String installedAppId);
    ZomeCallFfiParcel signZomeCall(in ZomeCallUnsignedFfiParcel request);
}
