//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[ensureAppWebsocket](ensure-app-websocket.md)

# ensureAppWebsocket

[androidJvm]\
suspend fun [ensureAppWebsocket](ensure-app-websocket.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)

Gets or creates an app websocket with authentication token.

#### Return

AppAuthFfi object containing websocket URL and authentication information

#### Parameters

androidJvm

| | |
|---|---|
| installedAppId | The ID of the app to create a websocket for |

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |