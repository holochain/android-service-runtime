//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[enableApp](enable-app.md)

# enableApp

[androidJvm]\
suspend fun [enableApp](enable-app.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [AppInfoFfi](../-app-info-ffi/index.md)

Enables an installed Holochain app.

#### Return

AppInfoFfi object with information about the enabled app

#### Parameters

androidJvm

| | |
|---|---|
| installedAppId | The ID of the app to enable |

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |