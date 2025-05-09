//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[RoleSettingsFfi](index.md)

# RoleSettingsFfi

sealed class [RoleSettingsFfi](index.md)

#### Inheritors

| |
|---|
| [UseExisting](-use-existing/index.md) |
| [Provisioned](-provisioned/index.md) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |
| [Provisioned](-provisioned/index.md) | [androidJvm]<br>data class [Provisioned](-provisioned/index.md)(val membraneProof: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html)?, val modifiers: [DnaModifiersOptFfi](../-dna-modifiers-opt-ffi/index.md)?) : [RoleSettingsFfi](index.md) |
| [UseExisting](-use-existing/index.md) | [androidJvm]<br>data class [UseExisting](-use-existing/index.md)(val cellId: [CellIdFfi](../-cell-id-ffi/index.md)) : [RoleSettingsFfi](index.md) |

## Functions

| Name | Summary |
|---|---|
| [write](../-role-settings-ffi-parceler/write.md) | [androidJvm]<br>open override fun [RoleSettingsFfi](index.md).[write](../-role-settings-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |