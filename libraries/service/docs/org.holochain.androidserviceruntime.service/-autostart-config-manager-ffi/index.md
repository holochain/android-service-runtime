//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[AutostartConfigManagerFfi](index.md)

# AutostartConfigManagerFfi

[androidJvm]\
open class [AutostartConfigManagerFfi](index.md) : [Disposable](../-disposable/index.md), [AutoCloseable](https://developer.android.com/reference/kotlin/java/lang/AutoCloseable.html), [AutostartConfigManagerFfiInterface](../-autostart-config-manager-ffi-interface/index.md)

## Constructors

| | |
|---|---|
| [AutostartConfigManagerFfi](-autostart-config-manager-ffi.md) | [androidJvm]<br>constructor(pointer: Pointer)constructor(noPointer: NoPointer)<br>This constructor can be used to instantiate a fake object. Only used for tests. Any attempt to actually use an object constructed this way will fail as there is no connected Rust object.<br>constructor(dataRootPath: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Functions

| Name | Summary |
|---|---|
| [close](close.md) | [androidJvm]<br>@[Synchronized](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.jvm/-synchronized/index.html)<br>open override fun [close](close.md)() |
| [destroy](destroy.md) | [androidJvm]<br>open override fun [destroy](destroy.md)() |
| [disable](disable.md) | [androidJvm]<br>open override fun [disable](disable.md)() |
| [enable](enable.md) | [androidJvm]<br>open override fun [enable](enable.md)(config: RuntimeNetworkConfigFfi) |
| [getEnabledConfig](get-enabled-config.md) | [androidJvm]<br>open override fun [getEnabledConfig](get-enabled-config.md)(): RuntimeNetworkConfigFfi? |
| [uniffiClonePointer](uniffi-clone-pointer.md) | [androidJvm]<br>fun [uniffiClonePointer](uniffi-clone-pointer.md)(): Pointer |