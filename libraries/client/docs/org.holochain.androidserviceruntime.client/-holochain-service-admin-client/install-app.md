//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[installApp](install-app.md)

# installApp

[androidJvm]\
suspend fun [installApp](install-app.md)(payload: [InstallAppPayloadFfi](../-install-app-payload-ffi/index.md)): [AppInfoFfi](../-app-info-ffi/index.md)

Installs a Holochain app

#### Return

AppInfoFfi object with information about the installed app

#### Parameters

androidJvm

| | |
|---|---|
| payload | The installation payload containing the app bundle and configuration |

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |