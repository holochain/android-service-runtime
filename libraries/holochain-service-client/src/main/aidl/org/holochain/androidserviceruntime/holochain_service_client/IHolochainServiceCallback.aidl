package org.holochain.androidserviceruntime.holochain_service_client;

import org.holochain.androidserviceruntime.holochain_service_client.AppInfoFfiParcel;

interface IHolochainServiceCallback {
    void listApps(in List<AppInfoFfiParcel> response);
    void installApp(in AppInfoFfiParcel response);
}
