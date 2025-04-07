package org.holochain.androidserviceruntime.holochain_service_client;

import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceCallback;
import org.holochain.androidserviceruntime.holochain_service_client.InstallAppPayloadFfiParcel;
import org.holochain.androidserviceruntime.holochain_service_client.ZomeCallUnsignedFfiParcel;

interface IHolochainServiceApp {
    void installApp(IHolochainServiceCallback callback, in InstallAppPayloadFfiParcel request);
    void enableApp(IHolochainServiceCallback callback, String installedAppId);
    void isAppInstalled(IHolochainServiceCallback callback, String installedAppId);
    void ensureAppWebsocket(IHolochainServiceCallback callback, String installedAppId);
    void signZomeCall(IHolochainServiceCallback callback, in ZomeCallUnsignedFfiParcel request);
}
