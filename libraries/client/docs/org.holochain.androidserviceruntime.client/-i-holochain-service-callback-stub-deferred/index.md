//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[IHolochainServiceCallbackStubDeferred](index.md)

# IHolochainServiceCallbackStubDeferred

open class [IHolochainServiceCallbackStubDeferred](index.md)&lt;[T](index.md)&gt;

An extension of IHolochainServiceCallback.Stub

All functions in the AIDL interface are defined. Error callbacks are implemented, while success callbacks are placeholders.

This is intended to be extended, with the success callback overridden, for each service call.

#### Inheritors

| |
|---|
| [ListAppsCallbackDeferred](../-list-apps-callback-deferred/index.md) |
| [SetupAppCallbackDeferred](../-setup-app-callback-deferred/index.md) |
| [InstallAppCallbackDeferred](../-install-app-callback-deferred/index.md) |
| [UninstallAppCallbackDeferred](../-uninstall-app-callback-deferred/index.md) |
| [EnableAppCallbackDeferred](../-enable-app-callback-deferred/index.md) |
| [DisableAppCallbackDeferred](../-disable-app-callback-deferred/index.md) |
| [IsAppInstalledCallbackDeferred](../-is-app-installed-callback-deferred/index.md) |
| [EnsureAppWebsocketCallbackDeferred](../-ensure-app-websocket-callback-deferred/index.md) |
| [SignZomeCallCallbackDeferred](../-sign-zome-call-callback-deferred/index.md) |

## Constructors

| | |
|---|---|
| [IHolochainServiceCallbackStubDeferred](-i-holochain-service-callback-stub-deferred.md) | [androidJvm]<br>constructor() |

## Functions

| Name | Summary |
|---|---|
| [adminBinderUnauthorizedException](admin-binder-unauthorized-exception.md) | [androidJvm]<br>open fun [adminBinderUnauthorizedException](admin-binder-unauthorized-exception.md)(response: [AdminBinderUnauthorizedExceptionParcel](../-admin-binder-unauthorized-exception-parcel/index.md)) |
| [appBinderUnauthorizedException](app-binder-unauthorized-exception.md) | [androidJvm]<br>open fun [appBinderUnauthorizedException](app-binder-unauthorized-exception.md)(response: [AppBinderUnauthorizedExceptionParcel](../-app-binder-unauthorized-exception-parcel/index.md)) |
| [await](await.md) | [androidJvm]<br>suspend fun [await](await.md)(): [T](index.md) |
| [disableApp](disable-app.md) | [androidJvm]<br>open fun [disableApp](disable-app.md)() |
| [enableApp](enable-app.md) | [androidJvm]<br>open fun [enableApp](enable-app.md)(response: [AppInfoFfiParcel](../-app-info-ffi-parcel/index.md)) |
| [ensureAppWebsocket](ensure-app-websocket.md) | [androidJvm]<br>open fun [ensureAppWebsocket](ensure-app-websocket.md)(response: [AppAuthFfiParcel](../-app-auth-ffi-parcel/index.md)) |
| [installApp](install-app.md) | [androidJvm]<br>open fun [installApp](install-app.md)(response: [AppInfoFfiParcel](../-app-info-ffi-parcel/index.md)) |
| [isAppInstalled](is-app-installed.md) | [androidJvm]<br>open fun [isAppInstalled](is-app-installed.md)(response: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)) |
| [listApps](list-apps.md) | [androidJvm]<br>open fun [listApps](list-apps.md)(response: [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[AppInfoFfiParcel](../-app-info-ffi-parcel/index.md)&gt;) |
| [setupApp](setup-app.md) | [androidJvm]<br>open fun [setupApp](setup-app.md)(response: [AppAuthFfiParcel](../-app-auth-ffi-parcel/index.md)) |
| [signZomeCall](sign-zome-call.md) | [androidJvm]<br>open fun [signZomeCall](sign-zome-call.md)(response: [ZomeCallParamsSignedFfiParcel](../-zome-call-params-signed-ffi-parcel/index.md)) |
| [uninstallApp](uninstall-app.md) | [androidJvm]<br>open fun [uninstallApp](uninstall-app.md)() |