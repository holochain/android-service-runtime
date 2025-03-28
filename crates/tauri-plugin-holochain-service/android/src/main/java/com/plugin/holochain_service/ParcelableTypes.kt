package com.plugin.holochain_service

import kotlinx.parcelize.Parcelize
import kotlinx.parcelize.RawValue
import android.os.Parcelable
import android.os.SharedMemory
import android.system.OsConstants
import java.nio.ByteBuffer

@Parcelize
class AppInfoFfiParcel(val inner: @RawValue AppInfoFfi): Parcelable

@Parcelize
data class InstallAppPayloadFfiParcel(
    val sourceSharedMemory: SharedMemory,
    val installedAppId: String?,
    var networkSeed: String?,
    val rolesSettings: Map<String, @RawValue RoleSettingsFfi>?,
): Parcelable

fun InstallAppPayloadFfi.toParcel(): InstallAppPayloadFfiParcel {
    // Copy source bytes to shared memory
    // to avoid a TransactionTooLarge error which limits the size of IPC messages
    val sourceSharedMemory = SharedMemory.create(this.installedAppId, this.source.size)
    val appBundleSharedMemoryBuffer: ByteBuffer = sourceSharedMemory.mapReadWrite()
    appBundleSharedMemoryBuffer.put(this.source)
    sourceSharedMemory.setProtect(OsConstants.PROT_READ)

    return InstallAppPayloadFfiParcel(
        sourceSharedMemory,
        this.installedAppId,
        this.networkSeed,
        this.rolesSettings,
    )
}

fun InstallAppPayloadFfiParcel.fromParcel(): InstallAppPayloadFfi {
    // Read source bytes from shared memory
    // to avoid a TransactionTooLarge error which limits the size of IPC messages
    val sourceBuffer: ByteBuffer = this.sourceSharedMemory.mapReadOnly()
    val source: ByteArray = sourceBuffer.toByteArray()

    // Clear the shared memory
    SharedMemory.unmap(sourceBuffer)
    this.sourceSharedMemory.close()

    return InstallAppPayloadFfi(
        source,
        this.installedAppId,
        this.networkSeed,
        this.rolesSettings
    )
}

fun ByteBuffer.toByteArray(): ByteArray {
    return if (hasArray()) {
        array()
    } else {
        val bytes = ByteArray(remaining())
        get(bytes)
        bytes
    }
}

@Parcelize
class AppInfoStatusFfiParcel(val inner: @RawValue AppInfoStatusFfi): Parcelable

@Parcelize
class AppAuthenticationTokenIssuedFfiParcel(val inner: @RawValue AppAuthenticationTokenIssuedFfi): Parcelable

@Parcelize
class AppAuthFfiParcel(val inner: @RawValue AppAuthFfi): Parcelable

@Parcelize
class ZomeCallUnsignedFfiParcel(val inner: @RawValue ZomeCallUnsignedFfi): Parcelable

@Parcelize
class ZomeCallFfiParcel(val inner: @RawValue ZomeCallFfi): Parcelable
