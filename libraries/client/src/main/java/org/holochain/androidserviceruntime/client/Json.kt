/**
 * JSON Serialization of arbitrary objects and arrays.
 *
 * This is intended to be as generic as possible, but may not work for every type.
 * If you run into errors, you likely need to override handling of certain property types.
 * Types may cast it to a String if it doesn't know how to handle it specifically.
 *
 * Note that sealed classes must be matched explicitly for proper serialization.
 */
package org.holochain.androidserviceruntime.client

import android.util.Log
import org.json.JSONArray
import org.json.JSONObject
import kotlin.reflect.KProperty1
import kotlin.reflect.full.memberProperties

/**
 * Utility object for converting Kotlin objects to JSON representations.
 */
object Json {
    /**
     * Converts any Kotlin object to a JSON-compatible primitive or complex type.
     *
     * Handles various primitive types, collections, maps, and custom objects.
     * Special handling is provided for Rust-derived sealed classes.
     *
     * @param value The value to convert to a JSON-compatible form
     * @return A JSON-compatible representation of the input value
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    fun toJSONPrimitive(value: Any?): Any? =
        when (value) {
            null,
            Unit,
            -> null
            is String,
            is Int,
            is Double,
            is Boolean,
            -> value
            is Float -> value.toDouble()
            is Byte,
            is Short,
            is Long,
            -> value.toInt()
            is UByte -> value.toInt()
            is UShort -> value.toInt()
            is ULong -> value.toInt()
            is UInt -> value.toInt()
            is Enum<*> -> {
                value.name
            }
            is Map<*, *> -> {
                var map = HashMap<String, Any?>()
                value.forEach { entry ->
                    try {
                        map.put(entry.key as String, toJSONPrimitive(entry.value))
                    } catch (e: Exception) {
                        Log.e(
                            "toJSONObject",
                            "Error converting Map entry ${entry.key} with value ${entry.value} to JSONObject",
                            e,
                        )
                    }
                }
                JSONObject(map) as Any
            }
            is ByteArray -> {
                val byteCollection: MutableCollection<UByte> = value.toUByteArray().toMutableList()
                val jsValue =
                    try {
                        (byteCollection as? Collection<UByte>)?.toJSONArray()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting $value to toJSONArray", e)
                        null
                    }
                jsValue
            }
            is Collection<*> -> {
                val jsValue =
                    try {
                        (value.map { it as Any }).toJSONArray()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting $value to toJSONArray", e)
                        null
                    }
                jsValue
            }
            // Is this a known sealed class (i.e. converted from a Rust enum)?
            is AppInfoStatusFfi,
            is CellInfoFfi,
            is DisabledAppReasonFfi,
            is PausedAppReasonFfi,
            is RoleSettingsFfi,
            -> {
                val jsValue =
                    try {
                        value.toJSONObject()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting $value to JSONObject", e)
                        null
                    }
                jsValue?.put("type", value::class.simpleName)
                jsValue
            }
            else -> {
                val jsValue =
                    try {
                        value.toJSONObject()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting $value to JSONObject", e)
                        null
                    }
                jsValue
            }
        }

    /**
     * Extension function for Any? to convert to a JSON primitive.
     *
     * @return A JSON-compatible representation of this object
     */
    fun Any?.toJsonPrimitive(): Any? = toJSONPrimitive(this)

    /**
     * Converts any Kotlin object to a JSONObject.
     *
     * Uses reflection to get all properties of the object and converts each to a JSON-compatible form.
     *
     * @param data The object to convert
     * @return A JSONObject representation of the input object
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    inline fun <reified T : Any> toJSONObject(data: T): JSONObject {
        val obj = JSONObject()
        val properties = data::class.memberProperties
        for (property in properties) {
            val prop = property as? KProperty1<T, *>
            val value = prop?.get(data)
            obj.put(property.name, value.toJsonPrimitive())
        }
        return obj
    }

    /**
     * Converts a Collection to a JSONArray.
     *
     * Each element in the collection is converted to a JSON-compatible form.
     *
     * @param data The collection to convert
     * @return A JSONArray representation of the input collection
     */
    inline fun <reified T : Collection<Any>> toJSONArray(data: T): JSONArray {
        val arr = JSONArray()
        for (element in data) {
            arr.put(element.toJsonPrimitive())
        }
        return arr
    }
}

/**
 * Extension function to convert any object to a JSONObject.
 *
 * @return A JSONObject representation of this object
 */
fun Any.toJSONObject(): JSONObject = Json.toJSONObject(this)

/**
 * Extension function to convert a Collection to a JSONArray.
 *
 * @return A JSONArray representation of this collection
 */
fun Collection<Any>.toJSONArray(): JSONArray = Json.toJSONArray(this)

/**
 * Extension function to convert any object to a JSON string.
 *
 * @return A JSON string representation of this object
 */
fun Any.toJSONObjectString(): String = Json.toJSONObject(this).toString()

/**
 * Extension function to convert a Collection to a JSON string.
 *
 * @return A JSON string representation of this collection
 */
fun Collection<Any>.toJSONArrayString(): String = Json.toJSONArray(this).toString()
