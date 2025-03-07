package com.holochain_apps.holochain_service_types;

parcelable InstallAppPayloadFfiParcel;
parcelable ListInstalledAppsResponse;
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

interface IHolochainCallbackService {
    void installAppCallback(AppInfoFfiParcel response);
    void enableAppCallback(AppInfoFfiParcel response);
    void listAppsCallback(List<AppInfoFfiParcel> response);
    void ensureAppWebsocket(AppWebsocketFfiParcel response);
    void signZomeCall(ZomeCallFfiParcel response);
}
