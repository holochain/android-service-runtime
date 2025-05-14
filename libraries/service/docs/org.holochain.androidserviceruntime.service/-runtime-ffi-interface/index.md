//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[RuntimeFfiInterface](index.md)

# RuntimeFfiInterface

interface [RuntimeFfiInterface](index.md)

Slim wrapper around HolochainRuntime, with types compatible with Uniffi-generated FFI bindings.

#### Inheritors

| |
|---|
| [RuntimeFfi](../-runtime-ffi/index.md) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Functions

| Name | Summary |
|---|---|
| [authorizeAppClient](authorize-app-client.md) | [androidJvm]<br>abstract fun [authorizeAppClient](authorize-app-client.md)(clientId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Authorize a client to call the given app id |
| [disableApp](disable-app.md) | [androidJvm]<br>abstract suspend fun [disableApp](disable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Disable an installed app |
| [enableApp](enable-app.md) | [androidJvm]<br>abstract suspend fun [enableApp](enable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): AppInfoFfi<br>Enable an installed app |
| [ensureAppWebsocket](ensure-app-websocket.md) | [androidJvm]<br>abstract suspend fun [ensureAppWebsocket](ensure-app-websocket.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): AppAuthFfi<br>Get or create an app websocket with an authentication for the given app id |
| [installApp](install-app.md) | [androidJvm]<br>abstract suspend fun [installApp](install-app.md)(payload: InstallAppPayloadFfi): AppInfoFfi<br>Install an app |
| [isAppClientAuthorized](is-app-client-authorized.md) | [androidJvm]<br>abstract fun [isAppClientAuthorized](is-app-client-authorized.md)(clientId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)<br>Is this client authorized to call the given app id? |
| [isAppInstalled](is-app-installed.md) | [androidJvm]<br>abstract suspend fun [isAppInstalled](is-app-installed.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)<br>Is an app with the given installed_app_id installed on the conductor |
| [listApps](list-apps.md) | [androidJvm]<br>abstract suspend fun [listApps](list-apps.md)(): [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;AppInfoFfi&gt;<br>List apps installed on the conductor |
| [setupApp](setup-app.md) | [androidJvm]<br>abstract suspend fun [setupApp](setup-app.md)(payload: InstallAppPayloadFfi, enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): AppAuthFfi<br>Full process to setup an app |
| [signZomeCall](sign-zome-call.md) | [androidJvm]<br>abstract suspend fun [signZomeCall](sign-zome-call.md)(zomeCallUnsigned: ZomeCallUnsignedFfi): ZomeCallFfi<br>Sign a zome call |
| [stop](stop.md) | [androidJvm]<br>abstract suspend fun [stop](stop.md)()<br>Shutdown the holochain conductor |
| [uninstallApp](uninstall-app.md) | [androidJvm]<br>abstract suspend fun [uninstallApp](uninstall-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Uninstall an app |