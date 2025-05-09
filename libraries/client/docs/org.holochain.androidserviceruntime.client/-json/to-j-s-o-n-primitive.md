//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[Json](index.md)/[toJSONPrimitive](to-j-s-o-n-primitive.md)

# toJSONPrimitive

[androidJvm]\
fun [toJSONPrimitive](to-j-s-o-n-primitive.md)(value: [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)?): [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)?

Converts any Kotlin object to a JSON-compatible primitive or complex type.

Handles various primitive types, collections, maps, and custom objects. Special handling is provided for Rust-derived sealed classes.

#### Return

A JSON-compatible representation of the input value

#### Parameters

androidJvm

| | |
|---|---|
| value | The value to convert to a JSON-compatible form |