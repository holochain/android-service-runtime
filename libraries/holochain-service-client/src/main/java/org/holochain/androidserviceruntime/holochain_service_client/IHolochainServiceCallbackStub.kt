package org.holochain.androidserviceruntime.holochain_service_client

open class IHolochainServiceCallbackStub : IHolochainServiceCallback.Stub() {
  override fun listApps(response: List<AppInfoFfiParcel>) {}

  override fun setupApp(response: AppAuthFfiParcel) {}

  override fun installApp(response: AppInfoFfiParcel) {}

  override fun uninstallApp() {}

  override fun enableApp(response: AppInfoFfiParcel) {}

  override fun disableApp() {}

  override fun isAppInstalled(response: Boolean) {}

  override fun ensureAppWebsocket(response: AppAuthFfiParcel) {}

  override fun signZomeCall(response: ZomeCallFfiParcel) {}
}
