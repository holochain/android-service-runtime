//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAppClient](index.md)

# HolochainServiceAppClient

[androidJvm]\
class [HolochainServiceAppClient](index.md)(activity: [Activity](https://developer.android.com/reference/kotlin/android/app/Activity.html), serviceComponentName: [ComponentName](https://developer.android.com/reference/kotlin/android/content/ComponentName.html))

## Constructors

| | |
|---|---|
| [HolochainServiceAppClient](-holochain-service-app-client.md) | [androidJvm]<br>constructor(activity: [Activity](https://developer.android.com/reference/kotlin/android/app/Activity.html), serviceComponentName: [ComponentName](https://developer.android.com/reference/kotlin/android/content/ComponentName.html)) |

## Functions

| Name | Summary |
|---|---|
| [connect](connect.md) | [androidJvm]<br>fun [connect](connect.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Connects to the Holochain service using the App API for a specific app. |
| [connectSetupApp](connect-setup-app.md) | [androidJvm]<br>suspend fun [connectSetupApp](connect-setup-app.md)(installAppPayload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md), enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)<br>Connects to service, waits for connection to be ready, and sets up an app. |
| [enableApp](enable-app.md) | [androidJvm]<br>suspend fun [enableApp](enable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [AppInfoFfi](../-app-info-ffi/index.md)<br>Enables an installed Holochain app. |
| [setupApp](setup-app.md) | [androidJvm]<br>suspend fun [setupApp](setup-app.md)(installAppPayload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md), enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)<br>Complete process to setup a Holochain app. |
| [signZomeCall](sign-zome-call.md) | [androidJvm]<br>suspend fun [signZomeCall](sign-zome-call.md)(args: [ZomeCallUnsignedFfi](../-zome-call-unsigned-ffi/index.md)): [ZomeCallFfi](../-zome-call-ffi/index.md)<br>Signs a zome call with the agent's private key. |