//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[waitForConnectReady](wait-for-connect-ready.md)

# waitForConnectReady

[androidJvm]\
suspend fun [waitForConnectReady](wait-for-connect-ready.md)(timeoutMs: [Long](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-long/index.html) = 100)

Polls until connected to the service, or the timeout has elapsed.

#### Parameters

androidJvm

| | |
|---|---|
| timeoutMs | Maximum time to wait for connection in milliseconds (default: 100ms) |