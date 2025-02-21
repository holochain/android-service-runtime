package com.plugin.holochain_service

import java.nio.ByteBuffer

fun ByteBuffer.toByteArray(): ByteArray {
    return if (hasArray()) {
        array()
    } else {
        val bytes = ByteArray(remaining())
        get(bytes)
        bytes
    }
}