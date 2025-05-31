//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[RuntimeFfi](index.md)

# RuntimeFfi

[androidJvm]\
open class [RuntimeFfi](index.md) : [Disposable](../-disposable/index.md), [AutoCloseable](https://developer.android.com/reference/kotlin/java/lang/AutoCloseable.html), [RuntimeFfiInterface](../-runtime-ffi-interface/index.md)

Slim wrapper around HolochainRuntime, with types compatible with Uniffi-generated FFI bindings.

## Constructors

| | |
|---|---|
| [RuntimeFfi](-runtime-ffi.md) | [androidJvm]<br>constructor(pointer: Pointer)constructor(noPointer: NoPointer)<br>This constructor can be used to instantiate a fake object. Only used for tests. Any attempt to actually use an object constructed this way will fail as there is no connected Rust object. |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Functions

| Name | Summary |
|---|---|
| [authorizeAppClient](authorize-app-client.md) | [androidJvm]<br>open override fun [authorizeAppClient](authorize-app-client.md)(clientId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Authorize a client to call the given app id |
| [close](close.md) | [androidJvm]<br>@[Synchronized](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.jvm/-synchronized/index.html)<br>open override fun [close](close.md)() |
| [destroy](destroy.md) | [androidJvm]<br>open override fun [destroy](destroy.md)() |
| [disableApp](disable-app.md) | [androidJvm]<br>open suspend override fun [disableApp](disable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Disable an installed app |
| [enableApp](enable-app.md) | [androidJvm]<br>open suspend override fun [enableApp](enable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): AppInfoFfi<br>Enable an installed app |
| [ensureAppWebsocket](ensure-app-websocket.md) | [androidJvm]<br>open suspend override fun [ensureAppWebsocket](ensure-app-websocket.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): AppAuthFfi<br>Get or create an app websocket with an authentication for the given app id |
| [installApp](install-app.md) | [androidJvm]<br>open suspend override fun [installApp](install-app.md)(payload: InstallAppPayloadFfi): AppInfoFfi<br>Install an app |
| [isAppClientAuthorized](is-app-client-authorized.md) | [androidJvm]<br>open override fun [isAppClientAuthorized](is-app-client-authorized.md)(clientId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)<br>Is this client authorized to call the given app id? |
| [isAppInstalled](is-app-installed.md) | [androidJvm]<br>open suspend override fun [isAppInstalled](is-app-installed.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)<br>Is an app with the given installed_app_id installed on the conductor |
| [listApps](list-apps.md) | [androidJvm]<br>open suspend override fun [listApps](list-apps.md)(): [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;AppInfoFfi&gt;<br>List apps installed on the conductor |
| [setupApp](setup-app.md) | [androidJvm]<br>open suspend override fun [setupApp](setup-app.md)(payload: InstallAppPayloadFfi, enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): AppAuthFfi<br>Full process to setup an app |
| [signZomeCall](sign-zome-call.md) | [androidJvm]<br>open suspend override fun [signZomeCall](sign-zome-call.md)(zomeCallUnsigned: ZomeCallParamsFfi): ZomeCallParamsSignedFfi<br>Sign a zome call |
| [stop](stop.md) | [androidJvm]<br>open suspend override fun [stop](stop.md)()<br>Shutdown the holochain conductor |
| [uniffiClonePointer](uniffi-clone-pointer.md) | [androidJvm]<br>fun [uniffiClonePointer](uniffi-clone-pointer.md)(): Pointer |
| [uninstallApp](uninstall-app.md) | [androidJvm]<br>open suspend override fun [uninstallApp](uninstall-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))<br>Uninstall an app |