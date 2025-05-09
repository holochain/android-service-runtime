//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[RuntimeErrorFfi](index.md)

# RuntimeErrorFfi

sealed class [RuntimeErrorFfi](index.md) : [Exception](https://developer.android.com/reference/kotlin/java/lang/Exception.html)

#### Inheritors

| |
|---|
| [Runtime](-runtime/index.md) |
| [RuntimeNotStarted](-runtime-not-started/index.md) |
| [Config](-config/index.md) |
| [DecodeAppBundle](-decode-app-bundle/index.md) |

## Types

| Name | Summary |
|---|---|
| [Config](-config/index.md) | [androidJvm]<br>class [Config](-config/index.md)(message: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) : [RuntimeErrorFfi](index.md) |
| [DecodeAppBundle](-decode-app-bundle/index.md) | [androidJvm]<br>class [DecodeAppBundle](-decode-app-bundle/index.md)(message: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) : [RuntimeErrorFfi](index.md) |
| [ErrorHandler](-error-handler/index.md) | [androidJvm]<br>object [ErrorHandler](-error-handler/index.md) : UniffiRustCallStatusErrorHandler&lt;[RuntimeErrorFfi](index.md)&gt; |
| [Runtime](-runtime/index.md) | [androidJvm]<br>class [Runtime](-runtime/index.md)(message: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) : [RuntimeErrorFfi](index.md) |
| [RuntimeNotStarted](-runtime-not-started/index.md) | [androidJvm]<br>class [RuntimeNotStarted](-runtime-not-started/index.md)(message: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) : [RuntimeErrorFfi](index.md) |

## Properties

| Name | Summary |
|---|---|
| [cause](-decode-app-bundle/index.md#-654012527%2FProperties%2F275946699) | [androidJvm]<br>open val [cause](-decode-app-bundle/index.md#-654012527%2FProperties%2F275946699): [Throwable](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-throwable/index.html)? |
| [message](-decode-app-bundle/index.md#1824300659%2FProperties%2F275946699) | [androidJvm]<br>open val [message](-decode-app-bundle/index.md#1824300659%2FProperties%2F275946699): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)? |