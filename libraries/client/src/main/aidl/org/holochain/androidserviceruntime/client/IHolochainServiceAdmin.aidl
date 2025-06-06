package org.holochain.androidserviceruntime.client;

import org.holochain.androidserviceruntime.client.IHolochainServiceCallback;
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfiParcel;
import org.holochain.androidserviceruntime.client.ZomeCallParamsFfiParcel;

interface IHolochainServiceAdmin {
    void stop();
    boolean isReady();
    void setupApp(IHolochainServiceCallback callback, in InstallAppPayloadFfiParcel request, boolean enableAfterInstall);
    void installApp(IHolochainServiceCallback callback, in InstallAppPayloadFfiParcel request);
    void uninstallApp(IHolochainServiceCallback callback, String installedAppId);
    void enableApp(IHolochainServiceCallback callback, String installedAppId);
    void disableApp(IHolochainServiceCallback callback, String installedAppId);
    void listApps(IHolochainServiceCallback callback);
    void isAppInstalled(IHolochainServiceCallback callback, String installedAppId);
    void ensureAppWebsocket(IHolochainServiceCallback callback, String installedAppId);
    void signZomeCall(IHolochainServiceCallback callback, in ZomeCallParamsFfiParcel request);
}
