//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[RuntimeNetworkConfigFfi](index.md)

# RuntimeNetworkConfigFfi

[androidJvm]\
data class [RuntimeNetworkConfigFfi](index.md)(var bootstrapUrl: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var signalUrl: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var iceUrls: [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)&gt;)

## Constructors

| | |
|---|---|
| [RuntimeNetworkConfigFfi](-runtime-network-config-ffi.md) | [androidJvm]<br>constructor(bootstrapUrl: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), signalUrl: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), iceUrls: [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)&gt;) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Properties

| Name | Summary |
|---|---|
| [bootstrapUrl](bootstrap-url.md) | [androidJvm]<br>var [bootstrapUrl](bootstrap-url.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)<br>URL of the bootstrap server |
| [iceUrls](ice-urls.md) | [androidJvm]<br>var [iceUrls](ice-urls.md): [List](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-list/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)&gt;<br>URLs of ICE servers |
| [signalUrl](signal-url.md) | [androidJvm]<br>var [signalUrl](signal-url.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)<br>URL of the sbd server |

## Functions

| Name | Summary |
|---|---|
| [write](../-runtime-network-config-ffi-parceler/write.md) | [androidJvm]<br>open override fun [RuntimeNetworkConfigFfi](index.md).[write](../-runtime-network-config-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |