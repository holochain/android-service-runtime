//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[InstallAppPayloadFfi](index.md)

# InstallAppPayloadFfi

[androidJvm]\
data class [InstallAppPayloadFfi](index.md)(var source: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), var installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), var networkSeed: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)?, var rolesSettings: [Map](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-map/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), [RoleSettingsFfi](../-role-settings-ffi/index.md)&gt;?)

## Constructors

| | |
|---|---|
| [InstallAppPayloadFfi](-install-app-payload-ffi.md) | [androidJvm]<br>constructor(source: [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html), installedAppId: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), networkSeed: [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)?, rolesSettings: [Map](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-map/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), [RoleSettingsFfi](../-role-settings-ffi/index.md)&gt;?) |

## Types

| Name | Summary |
|---|---|
| [Companion](-companion/index.md) | [androidJvm]<br>object [Companion](-companion/index.md) |

## Properties

| Name | Summary |
|---|---|
| [installedAppId](installed-app-id.md) | [androidJvm]<br>var [installedAppId](installed-app-id.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html) |
| [networkSeed](network-seed.md) | [androidJvm]<br>var [networkSeed](network-seed.md): [String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html)? |
| [rolesSettings](roles-settings.md) | [androidJvm]<br>var [rolesSettings](roles-settings.md): [Map](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-map/index.html)&lt;[String](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-string/index.html), [RoleSettingsFfi](../-role-settings-ffi/index.md)&gt;? |
| [source](source.md) | [androidJvm]<br>var [source](source.md): [ByteArray](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-byte-array/index.html)<br>Raw bytes of encoded AppBundle |

## Functions

| Name | Summary |
|---|---|
| [write](../-install-app-payload-ffi-parceler/write.md) | [androidJvm]<br>open override fun [InstallAppPayloadFfi](index.md).[write](../-install-app-payload-ffi-parceler/write.md)(parcel: [Parcel](https://developer.android.com/reference/kotlin/android/os/Parcel.html), flags: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)) |