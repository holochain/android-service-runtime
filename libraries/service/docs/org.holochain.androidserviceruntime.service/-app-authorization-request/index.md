//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[AppAuthorizationRequest](index.md)

# AppAuthorizationRequest

[androidJvm]\
data class [AppAuthorizationRequest](index.md)(var clientPackageName: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html))

Represents a request from an Android app to access a Holochain app.

## Constructors

| | |
|---|---|
| [AppAuthorizationRequest](-app-authorization-request.md) | [androidJvm]<br>constructor(clientPackageName: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) |

## Properties

| Name | Summary |
|---|---|
| [clientPackageName](client-package-name.md) | [androidJvm]<br>var [clientPackageName](client-package-name.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)<br>The Android package name requesting the authorization |
| [installedAppId](installed-app-id.md) | [androidJvm]<br>var [installedAppId](installed-app-id.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)<br>The Holochain app ID that the client wants to access |