//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[IsAppInstalledCallbackDeferred](index.md)

# IsAppInstalledCallbackDeferred

[androidJvm]\
class [IsAppInstalledCallbackDeferred](index.md) : [IHolochainServiceCallbackStubDeferred](../-i-holochain-service-callback-stub-deferred/index.md)&lt;[Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)&gt;

## Constructors

| | |
|---|---|
| [IsAppInstalledCallbackDeferred](-is-app-installed-callback-deferred.md) | [androidJvm]<br>constructor() |

## Functions

| Name | Summary |
|---|---|
| [adminBinderUnauthorizedException](../-i-holochain-service-callback-stub-deferred/admin-binder-unauthorized-exception.md) | [androidJvm]<br>open fun [adminBinderUnauthorizedException](../-i-holochain-service-callback-stub-deferred/admin-binder-unauthorized-exception.md)(response: [AdminBinderUnauthorizedExceptionParcel](../-admin-binder-unauthorized-exception-parcel/index.md)) |
| [appBinderUnauthorizedException](../-i-holochain-service-callback-stub-deferred/app-binder-unauthorized-exception.md) | [androidJvm]<br>open fun [appBinderUnauthorizedException](../-i-holochain-service-callback-stub-deferred/app-binder-unauthorized-exception.md)(response: [AppBinderUnauthorizedExceptionParcel](../-app-binder-unauthorized-exception-parcel/index.md)) |
| [await](../-i-holochain-service-callback-stub-deferred/await.md) | [androidJvm]<br>suspend fun [await](../-i-holochain-service-callback-stub-deferred/await.md)(): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html) |
| [disableApp](../-i-holochain-service-callback-stub-deferred/disable-app.md) | [androidJvm]<br>open fun [disableApp](../-i-holochain-service-callback-stub-deferred/disable-app.md)() |
| [enableApp](../-i-holochain-service-callback-stub-deferred/enable-app.md) | [androidJvm]<br>open fun [enableApp](../-i-holochain-service-callback-stub-deferred/enable-app.md)(response: [AppInfoFfiParcel](../-app-info-ffi-parcel/index.md)) |
| [ensureAppWebsocket](../-i-holochain-service-callback-stub-deferred/ensure-app-websocket.md) | [androidJvm]<br>open fun [ensureAppWebsocket](../-i-holochain-service-callback-stub-deferred/ensure-app-websocket.md)(response: [AppAuthFfiParcel](../-app-auth-ffi-parcel/index.md)) |
| [installApp](../-i-holochain-service-callback-stub-deferred/install-app.md) | [androidJvm]<br>open fun [installApp](../-i-holochain-service-callback-stub-deferred/install-app.md)(response: [AppInfoFfiParcel](../-app-info-ffi-parcel/index.md)) |
| [isAppInstalled](is-app-installed.md) | [androidJvm]<br>open override fun [isAppInstalled](is-app-installed.md)(response: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)) |
| [listApps](../-i-holochain-service-callback-stub-deferred/list-apps.md) | [androidJvm]<br>open fun [listApps](../-i-holochain-service-callback-stub-deferred/list-apps.md)(response: [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[AppInfoFfiParcel](../-app-info-ffi-parcel/index.md)&gt;) |
| [setupApp](../-i-holochain-service-callback-stub-deferred/setup-app.md) | [androidJvm]<br>open fun [setupApp](../-i-holochain-service-callback-stub-deferred/setup-app.md)(response: [AppAuthFfiParcel](../-app-auth-ffi-parcel/index.md)) |
| [signZomeCall](../-i-holochain-service-callback-stub-deferred/sign-zome-call.md) | [androidJvm]<br>open fun [signZomeCall](../-i-holochain-service-callback-stub-deferred/sign-zome-call.md)(response: [ZomeCallParamsSignedFfiParcel](../-zome-call-params-signed-ffi-parcel/index.md)) |
| [uninstallApp](../-i-holochain-service-callback-stub-deferred/uninstall-app.md) | [androidJvm]<br>open fun [uninstallApp](../-i-holochain-service-callback-stub-deferred/uninstall-app.md)() |