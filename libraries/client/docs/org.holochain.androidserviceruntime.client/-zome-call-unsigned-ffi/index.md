//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[ZomeCallUnsignedFfi](index.md)

# ZomeCallUnsignedFfi

[androidJvm]\
data class [ZomeCallUnsignedFfi](index.md)(var provenance: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), var cellId: [CellIdFfi](../-cell-id-ffi/index.md), var zomeName: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var fnName: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var capSecret: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html)?, var payload: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), var nonce: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), var expiresAt: [Long](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-long/index.html))

## Constructors

| | |
|---|---|
| [ZomeCallUnsignedFfi](-zome-call-unsigned-ffi.md) | [androidJvm]<br>constructor(provenance: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), cellId: [CellIdFfi](../-cell-id-ffi/index.md), zomeName: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), fnName: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), capSecret: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html)?, payload: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), nonce: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), expiresAt: [Long](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-long/index.html)) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Properties

| Name | Summary |
|---|---|
| [capSecret](cap-secret.md) | [androidJvm]<br>var [capSecret](cap-secret.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html)? |
| [cellId](cell-id.md) | [androidJvm]<br>var [cellId](cell-id.md): [CellIdFfi](../-cell-id-ffi/index.md) |
| [expiresAt](expires-at.md) | [androidJvm]<br>var [expiresAt](expires-at.md): [Long](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-long/index.html) |
| [fnName](fn-name.md) | [androidJvm]<br>var [fnName](fn-name.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) |
| [nonce](nonce.md) | [androidJvm]<br>var [nonce](nonce.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html) |
| [payload](payload.md) | [androidJvm]<br>var [payload](payload.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html) |
| [provenance](provenance.md) | [androidJvm]<br>var [provenance](provenance.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html) |
| [zomeName](zome-name.md) | [androidJvm]<br>var [zomeName](zome-name.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) |

## Functions

| Name | Summary |
|---|---|
| [write](../-zome-call-unsigned-ffi-parceler/write.md) | [androidJvm]<br>open override fun [ZomeCallUnsignedFfi](index.md).[write](../-zome-call-unsigned-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |