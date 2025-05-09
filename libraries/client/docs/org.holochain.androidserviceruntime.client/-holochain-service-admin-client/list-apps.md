//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[listApps](list-apps.md)

# listApps

[androidJvm]\
suspend fun [listApps](list-apps.md)(): [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[AppInfoFfi](../-app-info-ffi/index.md)&gt;

Lists all installed Holochain apps in the conductor.

#### Return

List of AppInfoFfi objects containing information about all installed apps

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |