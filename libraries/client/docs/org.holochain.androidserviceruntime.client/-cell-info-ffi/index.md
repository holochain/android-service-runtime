//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[CellInfoFfi](index.md)

# CellInfoFfi

sealed class [CellInfoFfi](index.md)

#### Inheritors

| |
|---|
| [Provisioned](-provisioned/index.md) |
| [Cloned](-cloned/index.md) |
| [Stem](-stem/index.md) |

## Types

| Name | Summary |
|---|---|
| [Cloned](-cloned/index.md) | [androidJvm]<br>data class [Cloned](-cloned/index.md)(val v1: [ClonedCellFfi](../-cloned-cell-ffi/index.md)) : [CellInfoFfi](index.md) |
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |
| [Provisioned](-provisioned/index.md) | [androidJvm]<br>data class [Provisioned](-provisioned/index.md)(val v1: [ProvisionedCellFfi](../-provisioned-cell-ffi/index.md)) : [CellInfoFfi](index.md) |
| [Stem](-stem/index.md) | [androidJvm]<br>data class [Stem](-stem/index.md)(val v1: [StemCellFfi](../-stem-cell-ffi/index.md)) : [CellInfoFfi](index.md) |

## Functions

| Name | Summary |
|---|---|
| [write](../-cell-info-ffi-parceler/write.md) | [androidJvm]<br>open override fun [CellInfoFfi](index.md).[write](../-cell-info-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |