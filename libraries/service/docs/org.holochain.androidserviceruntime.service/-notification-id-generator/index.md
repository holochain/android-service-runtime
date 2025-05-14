//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[NotificationIdGenerator](index.md)

# NotificationIdGenerator

class [NotificationIdGenerator](index.md)(initialValue: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html) = 0)

Generates sequential, thread-safe notification IDs.

Used to create unique IDs for Android notifications.

#### Parameters

androidJvm

| | |
|---|---|
| initialValue | The starting value for the notification IDs (default: 0) |

## Constructors

| | |
|---|---|
| [NotificationIdGenerator](-notification-id-generator.md) | [androidJvm]<br>constructor(initialValue: [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html) = 0) |

## Functions

| Name | Summary |
|---|---|
| [next](next.md) | [androidJvm]<br>fun [next](next.md)(): [Int](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-int/index.html)<br>Gets the next available notification ID. |