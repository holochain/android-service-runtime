//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[AppBinderUnauthorizedException](index.md)

# AppBinderUnauthorizedException

class [AppBinderUnauthorizedException](index.md)(message: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) = &quot;AppBinderUnauthorized&quot;) : [Exception](https://developer.android.com/reference/kotlin/java/lang/Exception.html)

Exception thrown when a client is not authorized to use the App API for a specific app.

#### Parameters

androidJvm

| | |
|---|---|
| message | Error message (defaults to &quot;AppBinderUnauthorized&quot;) |

## Constructors

| | |
|---|---|
| [AppBinderUnauthorizedException](-app-binder-unauthorized-exception.md) | [androidJvm]<br>constructor(message: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) = &quot;AppBinderUnauthorized&quot;) |

## Properties

| Name | Summary |
|---|---|
| [cause](../-internal-exception/index.md#-654012527%2FProperties%2F275946699) | [androidJvm]<br>open val [cause](../-internal-exception/index.md#-654012527%2FProperties%2F275946699): [Throwable](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-throwable/index.html)? |
| [message](../-internal-exception/index.md#1824300659%2FProperties%2F275946699) | [androidJvm]<br>open val [message](../-internal-exception/index.md#1824300659%2FProperties%2F275946699): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)? |

## Functions

| Name | Summary |
|---|---|
| [write](../-app-binder-unauthorized-exception-parceler/write.md) | [androidJvm]<br>open override fun [AppBinderUnauthorizedException](index.md).[write](../-app-binder-unauthorized-exception-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |