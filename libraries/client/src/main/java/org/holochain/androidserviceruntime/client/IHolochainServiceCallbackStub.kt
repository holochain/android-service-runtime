package org.holochain.androidserviceruntime.client

import android.util.Log
import kotlinx.coroutines.CompletableDeferred

/**
 An extension of IHolochainServiceCallback.Stub

 All functions in the AIDL interface are defined.
 Error callbacks are implemented, while success callbacks are placeholders.

 This is intended to be extended, with the success callback overridden, for each service call.
 */
open class IHolochainServiceCallbackStubDeferred<T> : IHolochainServiceCallback.Stub() {
    internal val deferred = CompletableDeferred<T>()
    internal val logTag = "IHolochainServiceCallbackStubDeferred"

    suspend fun await(): T = this.deferred.await()

    /*
        Error responses
     */
    override fun appBinderUnauthorizedException(response: AppBinderUnauthorizedExceptionParcel) {
        Log.d(logTag, "appBinderUnauthorizedException")
        deferred.completeExceptionally(response.inner)
    }

    override fun adminBinderUnauthorizedException(response: AdminBinderUnauthorizedExceptionParcel) {
        Log.d(logTag, "adminBinderUnauthorizedException")
        this.deferred.completeExceptionally(response.inner)
    }

    /*
        Success response placeholders.

        This is necessary to ensure we have defined *all* the functions in the AIDL interface, even if we're not using them.
     */
    override fun listApps(response: List<AppInfoFfiParcel>) {}

    override fun setupApp(response: AppAuthFfiParcel) {}

    override fun installApp(response: AppInfoFfiParcel) {}

    override fun uninstallApp() {}

    override fun enableApp(response: AppInfoFfiParcel) {}

    override fun disableApp() {}

    override fun isAppInstalled(response: Boolean) {}

    override fun ensureAppWebsocket(response: AppAuthFfiParcel) {}

    override fun signZomeCall(response: ZomeCallParamsSignedFfiParcel) {}
}

/*
    Classes that override the Callback, and complete the Deferred with the result.

    A Deferred is akin to a Promise or Future.
 */
class ListAppsCallbackDeferred : IHolochainServiceCallbackStubDeferred<List<AppInfoFfi>>() {
    override fun listApps(response: List<AppInfoFfiParcel>) {
        Log.d(logTag, "listApps")
        deferred.complete(response.map { it.inner })
    }
}

class SetupAppCallbackDeferred : IHolochainServiceCallbackStubDeferred<AppAuthFfi>() {
    override fun setupApp(response: AppAuthFfiParcel) {
        Log.d(logTag, "setupApp")
        deferred.complete(response.inner)
    }
}

class InstallAppCallbackDeferred : IHolochainServiceCallbackStubDeferred<AppInfoFfi>() {
    override fun installApp(response: AppInfoFfiParcel) {
        Log.d(logTag, "installApp")
        deferred.complete(response.inner)
    }
}

class UninstallAppCallbackDeferred : IHolochainServiceCallbackStubDeferred<Unit>() {
    override fun uninstallApp() {
        Log.d(logTag, "uninstallApp")
        deferred.complete(Unit)
    }
}

class EnableAppCallbackDeferred : IHolochainServiceCallbackStubDeferred<AppInfoFfi>() {
    override fun enableApp(response: AppInfoFfiParcel) {
        Log.d(logTag, "enableApp")
        deferred.complete(response.inner)
    }
}

class DisableAppCallbackDeferred : IHolochainServiceCallbackStubDeferred<Unit>() {
    override fun disableApp() {
        Log.d(logTag, "disableApp")
        deferred.complete(Unit)
    }
}

class IsAppInstalledCallbackDeferred : IHolochainServiceCallbackStubDeferred<Boolean>() {
    override fun isAppInstalled(response: Boolean) {
        Log.d(logTag, "isAppInstalled")
        deferred.complete(response)
    }
}

class EnsureAppWebsocketCallbackDeferred : IHolochainServiceCallbackStubDeferred<AppAuthFfi>() {
    override fun ensureAppWebsocket(response: AppAuthFfiParcel) {
        Log.d(logTag, "ensureAppWebsocket")
        deferred.complete(response.inner)
    }
}

class SignZomeCallCallbackDeferred : IHolochainServiceCallbackStubDeferred<ZomeCallParamsSignedFfi>() {
    override fun signZomeCall(response: ZomeCallParamsSignedFfiParcel) {
        Log.d(logTag, "signZomeCall")
        deferred.complete(response.inner)
    }
}
