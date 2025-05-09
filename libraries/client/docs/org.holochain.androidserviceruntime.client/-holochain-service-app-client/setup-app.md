//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAppClient](index.md)/[setupApp](setup-app.md)

# setupApp

[androidJvm]\
suspend fun [setupApp](setup-app.md)(installAppPayload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md), enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)

Complete process to setup a Holochain app.

This method will:

1. 
   Check if the app is installed
2. 
   If not installed, install it
3. 
   Optionally enable it
4. 
   Ensure there is an app websocket available for it

If an app is already installed, it will not be enabled automatically. It is only enabled after a successful install. The reasoning is that if an app is disabled after initial installation,

#### Return

AppAuthFfi object containing authentication and connection information

#### Parameters

androidJvm

| | |
|---|---|
| installAppPayload | The payload containing app installation data |
| enableAfterInstall | Whether to enable the app after installation |

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |