package org.holochain.androidserviceruntime.holochain_service_client;

import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceCallback;
import org.holochain.androidserviceruntime.holochain_service_client.InstallAppPayloadFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfiParcel;

interface IHolochainService {
    void stop();
    boolean isReady();
    void installApp(IHolochainServiceCallback callback, in InstallAppPayloadFfiParcel request);
    void uninstallApp(IHolochainServiceCallback callback, String installedAppId);
    void enableApp(IHolochainServiceCallback callback, String installedAppId);
    void disableApp(IHolochainServiceCallback callback, String installedAppId);
    void listApps(IHolochainServiceCallback callback);
    void isAppInstalled(IHolochainServiceCallback callback, String installedAppId);
    void ensureAppWebsocket(IHolochainServiceCallback callback, String installedAppId);
    void signZomeCall(IHolochainServiceCallback callback, in ZomeCallUnsignedFfiParcel request);
}
