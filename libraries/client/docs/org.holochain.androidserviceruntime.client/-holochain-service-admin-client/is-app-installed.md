//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[isAppInstalled](is-app-installed.md)

# isAppInstalled

[androidJvm]\
suspend fun [isAppInstalled](is-app-installed.md)(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)

Checks if an app with the given app ID is installed.

#### Return

true if the app is installed, false otherwise

#### Parameters

androidJvm

| | |
|---|---|
| installedAppId | The ID of the app to check |

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |