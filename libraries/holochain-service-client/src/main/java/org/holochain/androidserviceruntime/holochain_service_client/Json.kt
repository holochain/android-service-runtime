/// JSON Serialization of arbitrary objects and arrays
///
/// This is intended to be as generic as possible, but may not work for every type.
/// If you run into errors, you likely need to override handling of certain property types.
/// Types may cast it to a String if it doesn't know how to handle it specifically.
///
/// Note that sealed classes must be matched explicitly
package org.holochain.androidserviceruntime.holochain_service_client

import kotlin.reflect.full.memberProperties
import kotlin.reflect.KProperty1
import android.util.Log
import org.holochain.androidserviceruntime.holochain_service_client.AppInfoStatusFfi
import org.holochain.androidserviceruntime.holochain_service_client.CellInfoFfi
import org.holochain.androidserviceruntime.holochain_service_client.DisabledAppReasonFfi
import org.holochain.androidserviceruntime.holochain_service_client.PausedAppReasonFfi
import org.holochain.androidserviceruntime.holochain_service_client.RoleSettingsFfi
import org.json.JSONArray
import org.json.JSONObject

object Json {
    /// Convert Any object to a JSObject
    @OptIn(ExperimentalUnsignedTypes::class)
    inline fun <reified T : Any> toJSONObject(data: T): JSONObject {
        val obj = JSONObject()
        val properties = data::class.memberProperties
        for (property in properties) {
            val prop = property as? KProperty1<T, *>
            val value = prop?.get(data)
            when (value) {
                is String, is Int, is Long, is Double, is Boolean, is ULong, is UInt  -> obj.put(property.name, value)
                is Enum<*> -> obj.put(property.name, value.name)
                null -> obj.put(property.name, null)
                is Map<*,*> -> {
                    var map = HashMap<String, Any>()
                    value.forEach { entry ->
                        try {
                            val entryJsValue = when (entry.value) {
                                is Collection<*> -> {
                                    ((entry.value as Collection<*>).map { it as Any}).toJSONArray()
                                }
                                else -> {
                                    entry.value?.toJSONObject()
                                }
                            }
                            map.put(entry.key as String, entryJsValue as Any)
                        } catch (e: Exception) {
                            Log.e("toJSONObject", "Error converting Map entry ${entry.key} with value ${entry.value} to JSObject", e)
                        }
                    }
                    obj.put(property.name, JSONObject(map) as Any)
                }
                is ByteArray -> {
                    val byteCollection: MutableCollection<UByte> = value.toUByteArray().toMutableList()
                    val jsValue = try {
                        (byteCollection as? Collection<UByte>)?.toJSONArray()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting property ${property.name} to toJSONArray", e)
                        null
                    }
                    obj.put(property.name, jsValue)
                }
                is Collection<*> -> {
                    val jsValue = try {
                        (value.map { it as Any }).toJSONArray()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting property ${property.name} to toJSONArray", e)
                        null
                    }
                    obj.put(property.name, jsValue)
                }
                // Is this a known sealed class (i.e. converted from a Rust enum)?
                is AppInfoStatusFfi, is CellInfoFfi, is DisabledAppReasonFfi, is PausedAppReasonFfi, is RoleSettingsFfi -> {
                    val jsValue = try {
                        value.toJSONObject()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting property ${property.name} to JSObject", e)
                        null
                    }
                    jsValue?.put("type", value::class.simpleName)
                    obj.put(property.name, jsValue)
                }
                else -> {
                    val jsValue = try {
                        value.toJSONObject()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting property ${property.name} to JSObject", e)
                        null
                    }
                    obj.put(property.name, jsValue)
                }
            }
        }
        return obj
    }

    /// Convert Collection<Any> to a JSArray
    inline fun <reified T : Collection<Any>> toJSONArray(data: T): JSONArray {
        val arr = JSONArray()
        for (element in data) {
            when (element) {
                is String, is Int, is Long, is Double, is Float, is Boolean, is Byte, is ULong, is UInt -> arr.put(element)
                is UByte -> arr.put(element.toInt())
                is Enum<*> -> arr.put(element.name)
                is Map<*,*> -> {
                    Log.d("toJSONArray", "Element $element is map")
                    var map = HashMap<String, Any>()
                    element.forEach { entry ->
                        try {
                            val entryJsValue = when (entry.value) {
                                is Collection<*> -> {
                                    ((entry.value as Collection<*>).map { it as Any}).toJSONArray()
                                }
                                else -> {
                                    entry.value?.toJSONObject()
                                }
                            }
                            map.put(entry.key as String, entryJsValue as Any)
                        } catch (e: Exception) {
                            Log.e("toJSONObject", "Error converting Map entry ${entry.key} with value ${entry.value} to JSObject", e)
                        }
                    }
                    arr.put(map as Any)
                }
                is Collection<*> -> {
                    val jsValue = try {
                        (element.map { it as Any }).toJSONArray()
                    } catch (e: Exception) {
                        Log.e("toJSONArray", "Error converting element $element to toJSONArray", e)
                        null
                    }
                    arr.put(jsValue)
                }
                // Is this a known sealed class (i.e. converted from a Rust enum)?
                is AppInfoStatusFfi, is CellInfoFfi, is DisabledAppReasonFfi, is PausedAppReasonFfi, is RoleSettingsFfi -> {
                    val jsValue = try {
                        element.toJSONObject()
                    } catch (e: Exception) {
                        Log.e("toJSONObject", "Error converting element $element to JSObject", e)
                        null
                    }
                    jsValue?.put("type", element::class.simpleName)
                    arr.put(jsValue)
                }
                else -> {
                    Log.d("toJSONArray", "Element $element is other")
                    val jsValue = try {
                        element.toJSONObject()
                    } catch (e: Exception) {
                        Log.e("toJSONArray", "Error converting element $element to toJSONObject", e)
                        null
                    }
                    arr.put(jsValue)
                }
            }
        }
        return arr
    }
}

fun Any.toJSONObject(): JSONObject = Json.toJSONObject(this)
fun Collection<Any>.toJSONArray(): JSONArray = Json.toJSONArray(this)

fun Any.toJSONObjectString(): String = Json.toJSONObject(this).toString()
fun Collection<Any>.toJSONArrayString(): String = Json.toJSONArray(this).toString()
