//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[Json](index.md)/[toJSONObject](to-j-s-o-n-object.md)

# toJSONObject

[androidJvm]\
inline fun &lt;[T](to-j-s-o-n-object.md) : [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)&gt; [toJSONObject](to-j-s-o-n-object.md)(data: [T](to-j-s-o-n-object.md)): [JSONObject](https://developer.android.com/reference/kotlin/org/json/JSONObject.html)

Converts any Kotlin object to a JSONObject.

Uses reflection to get all properties of the object and converts each to a JSON-compatible form.

#### Return

A JSONObject representation of the input object

#### Parameters

androidJvm

| | |
|---|---|
| data | The object to convert |