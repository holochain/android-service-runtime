//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[DisabledAppReasonFfi](index.md)

# DisabledAppReasonFfi

sealed class [DisabledAppReasonFfi](index.md)

#### Inheritors

| |
|---|
| [NeverStarted](-never-started/index.md) |
| [NotStartedAfterProvidingMemproofs](-not-started-after-providing-memproofs/index.md) |
| [DeletingAgentKey](-deleting-agent-key/index.md) |
| [User](-user/index.md) |
| [Error](-error/index.md) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |
| [DeletingAgentKey](-deleting-agent-key/index.md) | [androidJvm]<br>object [DeletingAgentKey](-deleting-agent-key/index.md) : [DisabledAppReasonFfi](index.md) |
| [Error](-error/index.md) | [androidJvm]<br>data class [Error](-error/index.md)(val v1: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) : [DisabledAppReasonFfi](index.md) |
| [NeverStarted](-never-started/index.md) | [androidJvm]<br>object [NeverStarted](-never-started/index.md) : [DisabledAppReasonFfi](index.md) |
| [NotStartedAfterProvidingMemproofs](-not-started-after-providing-memproofs/index.md) | [androidJvm]<br>object [NotStartedAfterProvidingMemproofs](-not-started-after-providing-memproofs/index.md) : [DisabledAppReasonFfi](index.md) |
| [User](-user/index.md) | [androidJvm]<br>object [User](-user/index.md) : [DisabledAppReasonFfi](index.md) |

## Functions

| Name | Summary |
|---|---|
| [write](../-disabled-app-reason-ffi-parceler/write.md) | [androidJvm]<br>open override fun [DisabledAppReasonFfi](index.md).[write](../-disabled-app-reason-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |