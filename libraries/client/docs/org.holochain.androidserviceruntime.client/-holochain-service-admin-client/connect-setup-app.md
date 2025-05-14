//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[connectSetupApp](connect-setup-app.md)

# connectSetupApp

[androidJvm]\
suspend fun [connectSetupApp](connect-setup-app.md)(installAppPayload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md), enableAfterInstall: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)): [AppAuthFfi](../-app-auth-ffi/index.md)

Connects to service, waits for connection to be ready, and sets up an app.

Convenience method that combines connect(), waitForConnectReady(), and setupApp() into a single call.

#### Return

AppAuthFfi object containing authentication and connection information

#### Parameters

androidJvm

| | |
|---|---|
| installAppPayload | The payload containing app installation data |
| enableAfterInstall | Whether to enable the app after installation |