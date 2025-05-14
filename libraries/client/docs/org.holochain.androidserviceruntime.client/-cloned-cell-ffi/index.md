//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[ClonedCellFfi](index.md)

# ClonedCellFfi

[androidJvm]\
data class [ClonedCellFfi](index.md)(var cellId: [CellIdFfi](../-cell-id-ffi/index.md), var cloneId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var originalDnaHash: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), var dnaModifiers: [DnaModifiersFfi](../-dna-modifiers-ffi/index.md), var name: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var enabled: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html))

## Constructors

| | |
|---|---|
| [ClonedCellFfi](-cloned-cell-ffi.md) | [androidJvm]<br>constructor(cellId: [CellIdFfi](../-cell-id-ffi/index.md), cloneId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), originalDnaHash: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), dnaModifiers: [DnaModifiersFfi](../-dna-modifiers-ffi/index.md), name: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), enabled: [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html)) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Properties

| Name | Summary |
|---|---|
| [cellId](cell-id.md) | [androidJvm]<br>var [cellId](cell-id.md): [CellIdFfi](../-cell-id-ffi/index.md) |
| [cloneId](clone-id.md) | [androidJvm]<br>var [cloneId](clone-id.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) |
| [dnaModifiers](dna-modifiers.md) | [androidJvm]<br>var [dnaModifiers](dna-modifiers.md): [DnaModifiersFfi](../-dna-modifiers-ffi/index.md) |
| [enabled](enabled.md) | [androidJvm]<br>var [enabled](enabled.md): [Boolean](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-boolean/index.html) |
| [name](name.md) | [androidJvm]<br>var [name](name.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) |
| [originalDnaHash](original-dna-hash.md) | [androidJvm]<br>var [originalDnaHash](original-dna-hash.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html) |

## Functions

| Name | Summary |
|---|---|
| [write](../-cloned-cell-ffi-parceler/write.md) | [androidJvm]<br>open override fun [ClonedCellFfi](index.md).[write](../-cloned-cell-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |