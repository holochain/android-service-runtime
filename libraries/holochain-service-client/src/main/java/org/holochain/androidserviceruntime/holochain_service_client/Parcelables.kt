package org.holochain.androidserviceruntime.holochain_service_client

import kotlinx.parcelize.Parcelize
import kotlinx.parcelize.RawValue
import android.os.Parcelable
import android.os.SharedMemory
import android.system.OsConstants
import java.nio.ByteBuffer
import kotlinx.parcelize.TypeParceler

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
@TypeParceler<AppInfoFfi, AppInfoFfiParceler>
data class AppInfoFfiParcel(val inner: AppInfoFfi) : Parcelable

@Parcelize
@TypeParceler<PausedAppReasonFfi, PausedAppReasonFfiParceler>
data class PausedAppReasonFfiParcel(val inner: PausedAppReasonFfi) : Parcelable

@Parcelize
@TypeParceler<DisabledAppReasonFfi, DisabledAppReasonFfiParceler>
data class DisabledAppReasonFfiParcel(val inner: DisabledAppReasonFfi) : Parcelable

@Parcelize
@TypeParceler<CellInfoFfi, CellInfoFfiParceler>
data class CellInfoFfiParcel(val inner: CellInfoFfi) : Parcelable

@Parcelize
@TypeParceler<CellIdFfi, CellIdFfiParceler>
data class CellIdFfiParcel(val inner: CellIdFfi) : Parcelable

@Parcelize
@TypeParceler<ProvisionedCellFfi, ProvisionedCellFfiParceler>
data class ProvisionedCellFfiParcel(val inner: ProvisionedCellFfi) : Parcelable

@Parcelize
@TypeParceler<ClonedCellFfi, ClonedCellFfiParceler>
data class ClonedCellFfiParcel(val inner: ClonedCellFfi) : Parcelable

@Parcelize
@TypeParceler<StemCellFfi, StemCellFfiParceler>
data class StemCellFfiParcel(val inner: StemCellFfi) : Parcelable

@Parcelize
@TypeParceler<AppInfoStatusFfi, AppInfoStatusFfiParceler>
data class AppInfoStatusFfiParcel(val inner: AppInfoStatusFfi) : Parcelable

@Parcelize
@TypeParceler<DnaModifiersFfi, DnaModifiersFfiParceler>
data class DnaModifiersFfiParcel(val inner: DnaModifiersFfi) : Parcelable

@Parcelize
@TypeParceler<DurationFfi, DurationFfiParceler>
data class DurationFfiParcel(val inner: DurationFfi) : Parcelable

@Parcelize
@TypeParceler<AppAuthenticationTokenIssuedFfi, AppAuthenticationTokenIssuedFfiParceler>
data class AppAuthenticationTokenIssuedFfiParcel(val inner: AppAuthenticationTokenIssuedFfi) : Parcelable

@Parcelize
@TypeParceler<AppAuthFfi, AppAuthFfiParceler>
data class AppAuthFfiParcel(val inner: AppAuthFfi) : Parcelable

@Parcelize
@TypeParceler<ZomeCallUnsignedFfi, ZomeCallUnsignedFfiParceler>
data class ZomeCallUnsignedFfiParcel(val inner: ZomeCallUnsignedFfi) : Parcelable

@Parcelize
@TypeParceler<ZomeCallFfi, ZomeCallFfiParceler>
data class ZomeCallFfiParcel(val inner: ZomeCallFfi) : Parcelable
