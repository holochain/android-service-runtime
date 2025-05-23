//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)

# HolochainServiceAdminClient

[androidJvm]\
class [HolochainServiceAdminClient](index.md)(activity: [Activity](https://developer.android.com/reference/kotlin/android/app/Activity.html), serviceComponentName: [ComponentName](https://developer.android.com/reference/kotlin/android/content/ComponentName.html))

## Constructors

| | |
|---|---|
| [HolochainServiceAdminClient](-holochain-service-admin-client.md) | [androidJvm]<br>constructor(activity: [Activity](https://developer.android.com/reference/kotlin/android/app/Activity.html), serviceComponentName: [ComponentName](https://developer.android.com/reference/kotlin/android/content/ComponentName.html)) |

## Functions

| Name | Summary |
|---|---|
| [connect](connect.md) | [androidJvm]<br>fun [connect](connect.md)()<br>Connect to the Holochain service using the Admin API. |
| [connectSetupApp](connect-setup-app.md) | [androidJvm]<br>suspend fun [connectSetupApp](connect-setup-app.md)(installAppPayload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md), enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)<br>Connects to service, waits for connection to be ready, and sets up an app. |
| [disableApp](disable-app.md) | [androidJvm]<br>suspend fun [disableApp](disable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Disables an installed Holochain app. |
| [enableApp](enable-app.md) | [androidJvm]<br>suspend fun [enableApp](enable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [AppInfoFfi](../-app-info-ffi/index.md)<br>Enables an installed Holochain app. |
| [ensureAppWebsocket](ensure-app-websocket.md) | [androidJvm]<br>suspend fun [ensureAppWebsocket](ensure-app-websocket.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)<br>Gets or creates an app websocket with authentication token. |
| [installApp](install-app.md) | [androidJvm]<br>suspend fun [installApp](install-app.md)(payload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md)): [AppInfoFfi](../-app-info-ffi/index.md)<br>Installs a Holochain app |
| [isAppInstalled](is-app-installed.md) | [androidJvm]<br>suspend fun [isAppInstalled](is-app-installed.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)<br>Checks if an app with the given app ID is installed. |
| [isReady](is-ready.md) | [androidJvm]<br>fun [isReady](is-ready.md)(): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)<br>Checks if the service is ready to receive calls. |
| [listApps](list-apps.md) | [androidJvm]<br>suspend fun [listApps](list-apps.md)(): [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[AppInfoFfi](../-app-info-ffi/index.md)&gt;<br>Lists all installed Holochain apps in the conductor. |
| [setupApp](setup-app.md) | [androidJvm]<br>suspend fun [setupApp](setup-app.md)(installAppPayload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md), enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)<br>Full process to setup a Holochain app. |
| [signZomeCall](sign-zome-call.md) | [androidJvm]<br>suspend fun [signZomeCall](sign-zome-call.md)(args: [ZomeCallUnsignedFfi](../-zome-call-unsigned-ffi/index.md)): [ZomeCallFfi](../-zome-call-ffi/index.md)<br>Signs a zome call with the agent's private key. |
| [start](start.md) | [androidJvm]<br>fun [start](start.md)(config: [RuntimeNetworkConfigFfi](../-runtime-network-config-ffi/index.md))<br>Start the Holochain service |
| [stop](stop.md) | [androidJvm]<br>fun [stop](stop.md)()<br>Stops the Holochain service. |
| [uninstallApp](uninstall-app.md) | [androidJvm]<br>suspend fun [uninstallApp](uninstall-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Uninstalls an installed Holochain app. |
| [waitForConnectReady](wait-for-connect-ready.md) | [androidJvm]<br>suspend fun [waitForConnectReady](wait-for-connect-ready.md)(timeoutMs: [Long](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-long/index.html) = 100)<br>Polls until connected to the service, or the timeout has elapsed. |