//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[Json](index.md)

# Json

[androidJvm]\
object [Json](index.md)

Utility object for converting Kotlin objects to JSON representations.

## Functions

| Name | Summary |
|---|---|
| [toJSONArray](to-j-s-o-n-array.md) | [androidJvm]<br>inline fun &lt;[T](to-j-s-o-n-array.md) : [Collection](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin.collections/-collection/index.html)&lt;[Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)&gt;&gt; [toJSONArray](to-j-s-o-n-array.md)(data: [T](to-j-s-o-n-array.md)): [JSONArray](https://developer.android.com/reference/kotlin/org/json/JSONArray.html)<br>Converts a Collection to a JSONArray. |
| [toJSONObject](to-j-s-o-n-object.md) | [androidJvm]<br>inline fun &lt;[T](to-j-s-o-n-object.md) : [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)&gt; [toJSONObject](to-j-s-o-n-object.md)(data: [T](to-j-s-o-n-object.md)): [JSONObject](https://developer.android.com/reference/kotlin/org/json/JSONObject.html)<br>Converts any Kotlin object to a JSONObject. |
| [toJSONPrimitive](to-j-s-o-n-primitive.md) | [androidJvm]<br>fun [toJSONPrimitive](to-j-s-o-n-primitive.md)(value: [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)?): [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)?<br>Converts any Kotlin object to a JSON-compatible primitive or complex type. |
| [toJsonPrimitive](to-json-primitive.md) | [androidJvm]<br>fun [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)?.[toJsonPrimitive](to-json-primitive.md)(): [Any](https://kotlinlang.org/api/core/kotlin-stdlib/kotlin/-any/index.html)?<br>Extension function for Any? to convert to a JSON primitive. |