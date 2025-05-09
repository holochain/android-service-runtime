//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[AppAuthorizationRequestNotification](index.md)

# AppAuthorizationRequestNotification

[androidJvm]\
data class [AppAuthorizationRequestNotification](index.md)(var request: [AppAuthorizationRequest](../-app-authorization-request/index.md), var notificationId: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html))

Associates an app authorization request with a notification ID.

## Constructors

| | |
|---|---|
| [AppAuthorizationRequestNotification](-app-authorization-request-notification.md) | [androidJvm]<br>constructor(request: [AppAuthorizationRequest](../-app-authorization-request/index.md), notificationId: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |

## Properties

| Name | Summary |
|---|---|
| [notificationId](notification-id.md) | [androidJvm]<br>var [notificationId](notification-id.md): [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)<br>The ID of the notification shown to the user |
| [request](request.md) | [androidJvm]<br>var [request](request.md): [AppAuthorizationRequest](../-app-authorization-request/index.md)<br>The authorization request |