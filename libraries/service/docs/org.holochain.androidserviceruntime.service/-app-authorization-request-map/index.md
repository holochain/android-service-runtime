//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[AppAuthorizationRequestMap](index.md)

# AppAuthorizationRequestMap

class [AppAuthorizationRequestMap](index.md)(initialNotificationId: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html))

A map of IDs to AppAuthorizationRequests and their associated notification IDs.

The &quot;approve&quot; intent must include the ID as extra data &quot;requestId&quot;, to indicate which Android package + Holochain app pair should be approved.

#### Parameters

androidJvm

| | |
|---|---|
| initialNotificationId | The starting notification ID to use |

## Constructors

| | |
|---|---|
| [AppAuthorizationRequestMap](-app-authorization-request-map.md) | [androidJvm]<br>constructor(initialNotificationId: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |

## Functions

| Name | Summary |
|---|---|
| [pop](pop.md) | [androidJvm]<br>fun [pop](pop.md)(uuid: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)): [AppAuthorizationRequestNotification](../-app-authorization-request-notification/index.md)?<br>Gets and removes a request from the map by its UUID. |
| [put](put.md) | [androidJvm]<br>fun [put](put.md)(request: [AppAuthorizationRequest](../-app-authorization-request/index.md)): [PutResponse](../-put-response/index.md)<br>Inserts a request into the map if not already included. |