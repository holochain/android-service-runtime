package org.holochain.androidserviceruntime.client;

import org.holochain.androidserviceruntime.client.IHolochainServiceCallback;
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfiParcel;
import org.holochain.androidserviceruntime.client.ZomeCallUnsignedFfiParcel;

interface IHolochainServiceApp {
    void setupApp(IHolochainServiceCallback callback, in InstallAppPayloadFfiParcel request, boolean enableAfterInstall);
    void enableApp(IHolochainServiceCallback callback);
    void ensureAppWebsocket(IHolochainServiceCallback callback);
    void signZomeCall(IHolochainServiceCallback callback, in ZomeCallUnsignedFfiParcel request);
}
