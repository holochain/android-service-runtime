//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[AppInfoStatusFfi](index.md)

# AppInfoStatusFfi

sealed class [AppInfoStatusFfi](index.md)

#### Inheritors

| |
|---|
| [Paused](-paused/index.md) |
| [Disabled](-disabled/index.md) |
| [Running](-running/index.md) |
| [AwaitingMemproofs](-awaiting-memproofs/index.md) |

## Types

| Name | Summary |
|---|---|
| [AwaitingMemproofs](-awaiting-memproofs/index.md) | [androidJvm]<br>object [AwaitingMemproofs](-awaiting-memproofs/index.md) : [AppInfoStatusFfi](index.md) |
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |
| [Disabled](-disabled/index.md) | [androidJvm]<br>data class [Disabled](-disabled/index.md)(val reason: [DisabledAppReasonFfi](../-disabled-app-reason-ffi/index.md)) : [AppInfoStatusFfi](index.md) |
| [Paused](-paused/index.md) | [androidJvm]<br>data class [Paused](-paused/index.md)(val reason: [PausedAppReasonFfi](../-paused-app-reason-ffi/index.md)) : [AppInfoStatusFfi](index.md) |
| [Running](-running/index.md) | [androidJvm]<br>object [Running](-running/index.md) : [AppInfoStatusFfi](index.md) |

## Functions

| Name | Summary |
|---|---|
| [write](../-app-info-status-ffi-parceler/write.md) | [androidJvm]<br>open override fun [AppInfoStatusFfi](index.md).[write](../-app-info-status-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |