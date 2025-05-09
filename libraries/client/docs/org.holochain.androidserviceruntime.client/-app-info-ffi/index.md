//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[AppInfoFfi](index.md)

# AppInfoFfi

[androidJvm]\
data class [AppInfoFfi](index.md)(var installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var cellInfo: [Map](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-map/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[CellInfoFfi](../-cell-info-ffi/index.md)&gt;&gt;, var status: [AppInfoStatusFfi](../-app-info-status-ffi/index.md), var agentPubKey: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html))

## Constructors

| | |
|---|---|
| [AppInfoFfi](-app-info-ffi.md) | [androidJvm]<br>constructor(installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), cellInfo: [Map](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-map/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[CellInfoFfi](../-cell-info-ffi/index.md)&gt;&gt;, status: [AppInfoStatusFfi](../-app-info-status-ffi/index.md), agentPubKey: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html)) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Properties

| Name | Summary |
|---|---|
| [agentPubKey](agent-pub-key.md) | [androidJvm]<br>var [agentPubKey](agent-pub-key.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html) |
| [cellInfo](cell-info.md) | [androidJvm]<br>var [cellInfo](cell-info.md): [Map](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-map/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[CellInfoFfi](../-cell-info-ffi/index.md)&gt;&gt; |
| [installedAppId](installed-app-id.md) | [androidJvm]<br>var [installedAppId](installed-app-id.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)<br>The unique identifier for an installed app in this conductor |
| [status](status.md) | [androidJvm]<br>var [status](status.md): [AppInfoStatusFfi](../-app-info-status-ffi/index.md) |

## Functions

| Name | Summary |
|---|---|
| [write](../-app-info-ffi-parceler/write.md) | [androidJvm]<br>open override fun [AppInfoFfi](index.md).[write](../-app-info-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |