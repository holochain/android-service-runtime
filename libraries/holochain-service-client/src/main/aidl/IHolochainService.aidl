package org.holochain.androidserviceruntime.holochain_service_client;

parcelable InstallAppPayloadFfiParcel;
parcelable AppInfoFfiParcel;
parcelable AppWebsocketFfiParcel;
parcelable ZomeCallUnsignedFfiParcel;
parcelable ZomeCallFfiParcel;

interface IHolochainService {
    void stop();
    AppInfoFfiParcel installApp(in InstallAppPayloadFfiParcel request);
    void uninstallApp(String installedAppId);
    AppInfoFfiParcel enableApp(String installedAppId);
    void disableApp(String installedAppId);
    List<AppInfoFfiParcel> listApps();
    boolean isAppInstalled(String installedAppId);
    AppWebsocketFfiParcel ensureAppWebsocket(String installedAppId);
    ZomeCallFfiParcel signZomeCall(in ZomeCallUnsignedFfiParcel request);
}
