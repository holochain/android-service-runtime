package org.holochain.androidserviceruntime.client

import android.os.Parcel
import android.os.SharedMemory
import android.system.OsConstants
import kotlinx.parcelize.Parceler
import kotlinx.parcelize.parcelableCreator
import java.nio.ByteBuffer

object RoleSettingsFfiParceler : Parceler<RoleSettingsFfi> {
    override fun create(parcel: Parcel): RoleSettingsFfi =
        when (parcel.readInt()) {
            1 ->
                RoleSettingsFfi.UseExisting(
                    parcelableCreator<CellIdFfiParcel>().createFromParcel(parcel).inner,
                )
            2 ->
                RoleSettingsFfi.Provisioned(
                    membraneProof = parcel.createByteArray() ?: ByteArray(0),
                    modifiers =
                        parcelableCreator<DnaModifiersOptFfiParcel>().createFromParcel(parcel).inner,
                )
            else -> throw IllegalArgumentException("Unknown RoleSettingsFfi type")
        }

    override fun RoleSettingsFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        when (this) {
            is RoleSettingsFfi.UseExisting -> {
                parcel.writeInt(1)
                CellIdFfiParcel(cellId).writeToParcel(parcel, flags)
            }
            is RoleSettingsFfi.Provisioned -> {
                parcel.writeInt(2)
                parcel.writeByteArray(membraneProof)

                if (modifiers != null) {
                    DnaModifiersOptFfiParcel(modifiers).writeToParcel(parcel, flags)
                }
            }
        }
    }
}

object InstallAppPayloadFfiParceler : Parceler<InstallAppPayloadFfi> {
    override fun create(parcel: Parcel): InstallAppPayloadFfi {
        // Read source bytes from shared memory
        // to avoid a TransactionTooLarge error which limits the size of IPC messages
        val sourceSharedMemory = parcelableCreator<SharedMemory>().createFromParcel(parcel)
        val sourceBuffer: ByteBuffer = sourceSharedMemory.mapReadOnly()
        val source: ByteArray = sourceBuffer.toByteArray()

        // Clear the shared memory
        SharedMemory.unmap(sourceBuffer)
        sourceSharedMemory.close()

        val installedAppId = parcel.readString()
        if (installedAppId == null) throw IllegalArgumentException("installedAppId must not be null")

        return InstallAppPayloadFfi(
            source,
            installedAppId!!,
            parcel.readString(),
            readRoleSettingsMap(parcel),
        )
    }

    override fun InstallAppPayloadFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        // Copy source bytes to shared memory
        // to avoid a TransactionTooLarge error which limits the size of IPC messages
        val sourceSharedMemory = SharedMemory.create(installedAppId, source.size)
        val appBundleSharedMemoryBuffer: ByteBuffer = sourceSharedMemory.mapReadWrite()
        appBundleSharedMemoryBuffer.put(this.source)
        sourceSharedMemory.setProtect(OsConstants.PROT_READ)

        sourceSharedMemory.writeToParcel(parcel, flags)
        parcel.writeString(installedAppId)
        parcel.writeString(networkSeed)

        if (rolesSettings != null) {
            writeRoleSettingsMap(parcel, rolesSettings!!, flags)
        }
    }

    private fun readRoleSettingsMap(parcel: Parcel): Map<String, RoleSettingsFfi>? {
        val size = parcel.readInt()

        if (size > 0) {
            val result = mutableMapOf<String, RoleSettingsFfi>()

            repeat(size) {
                val key = parcel.readString() ?: ""
                result[key] = parcelableCreator<RoleSettingsFfiParcel>().createFromParcel(parcel).inner
            }

            return result
        } else {
            return null
        }
    }

    private fun writeRoleSettingsMap(
        parcel: Parcel,
        map: Map<String, RoleSettingsFfi>?,
        flags: Int,
    ) {
        if (map != null) {
            parcel.writeInt(map.size)

            map.forEach { (key, value) ->
                parcel.writeString(key)
                RoleSettingsFfiParcel(value).writeToParcel(parcel, flags)
            }
        } else {
            parcel.writeInt(0)
        }
    }

    private fun ByteBuffer.toByteArray(): ByteArray =
        if (hasArray()) {
            array()
        } else {
            val bytes = ByteArray(remaining())
            get(bytes)
            bytes
        }
}

object AppInfoFfiParceler : Parceler<AppInfoFfi> {
    override fun create(parcel: Parcel): AppInfoFfi =
        AppInfoFfi(
            installedAppId = parcel.readString() ?: "",
            cellInfo = readCellInfoMap(parcel),
            status = parcelableCreator<AppInfoStatusFfiParcel>().createFromParcel(parcel).inner,
            agentPubKey = parcel.createByteArray() ?: ByteArray(0),
        )

    override fun AppInfoFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeString(installedAppId)
        writeCellInfoMap(parcel, cellInfo, flags)
        AppInfoStatusFfiParcel(status).writeToParcel(parcel, flags)
        parcel.writeByteArray(agentPubKey)
    }

    private fun readCellInfoMap(parcel: Parcel): Map<String, List<CellInfoFfi>> {
        val size = parcel.readInt()
        val result = mutableMapOf<String, List<CellInfoFfi>>()

        repeat(size) {
            val key = parcel.readString() ?: ""
            val listSize = parcel.readInt()
            val cellInfoList = mutableListOf<CellInfoFfi>()

            repeat(listSize) {
                cellInfoList.add(parcelableCreator<CellInfoFfiParcel>().createFromParcel(parcel).inner)
            }

            result[key] = cellInfoList
        }

        return result
    }

    private fun writeCellInfoMap(
        parcel: Parcel,
        map: Map<String, List<CellInfoFfi>>,
        flags: Int,
    ) {
        parcel.writeInt(map.size)

        map.forEach { (key, value) ->
            parcel.writeString(key)
            parcel.writeInt(value.size)

            value.forEach { cellInfo -> CellInfoFfiParcel(cellInfo).writeToParcel(parcel, flags) }
        }
    }
}

object AppInfoStatusFfiParceler : Parceler<AppInfoStatusFfi> {
    override fun create(parcel: Parcel): AppInfoStatusFfi =
        when (parcel.readInt()) {
            1 ->
                AppInfoStatusFfi.Paused(
                    parcelableCreator<PausedAppReasonFfiParcel>().createFromParcel(parcel).inner,
                )
            2 ->
                AppInfoStatusFfi.Disabled(
                    parcelableCreator<DisabledAppReasonFfiParcel>().createFromParcel(parcel).inner,
                )
            3 -> AppInfoStatusFfi.Running
            4 -> AppInfoStatusFfi.AwaitingMemproofs
            else -> throw IllegalArgumentException("Unknown AppInfoStatusFfi type")
        }

    override fun AppInfoStatusFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        when (this) {
            is AppInfoStatusFfi.Paused -> {
                parcel.writeInt(1)
                PausedAppReasonFfiParcel(reason).writeToParcel(parcel, flags)
            }
            is AppInfoStatusFfi.Disabled -> {
                parcel.writeInt(2)
                DisabledAppReasonFfiParcel(reason).writeToParcel(parcel, flags)
            }
            is AppInfoStatusFfi.Running -> parcel.writeInt(3)
            is AppInfoStatusFfi.AwaitingMemproofs -> parcel.writeInt(4)
        }
    }
}

object AppAuthenticationTokenIssuedFfiParceler : Parceler<AppAuthenticationTokenIssuedFfi> {
    override fun create(parcel: Parcel): AppAuthenticationTokenIssuedFfi =
        AppAuthenticationTokenIssuedFfi(
            token = parcel.createByteArray() ?: ByteArray(0),
            expiresAt = if (parcel.readInt() == 1) parcel.readLong() else null,
        )

    override fun AppAuthenticationTokenIssuedFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeByteArray(token)
        if (expiresAt != null) {
            parcel.writeInt(1)
            parcel.writeLong(expiresAt!!)
        } else {
            parcel.writeInt(0)
        }
    }
}

object AppAuthFfiParceler : Parceler<AppAuthFfi> {
    override fun create(parcel: Parcel): AppAuthFfi =
        AppAuthFfi(
            authentication =
                parcelableCreator<AppAuthenticationTokenIssuedFfiParcel>()
                    .createFromParcel(parcel)
                    .inner,
            port = parcel.readInt().toUShort(),
        )

    override fun AppAuthFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        AppAuthenticationTokenIssuedFfiParcel(authentication).writeToParcel(parcel, flags)
        parcel.writeInt(port.toInt())
    }
}

object ZomeCallUnsignedFfiParceler : Parceler<ZomeCallUnsignedFfi> {
    override fun create(parcel: Parcel): ZomeCallUnsignedFfi =
        ZomeCallUnsignedFfi(
            provenance = parcel.createByteArray() ?: ByteArray(0),
            cellId = parcelableCreator<CellIdFfiParcel>().createFromParcel(parcel).inner,
            zomeName = parcel.readString() ?: "",
            fnName = parcel.readString() ?: "",
            capSecret = if (parcel.readInt() == 1) parcel.createByteArray() else null,
            payload = parcel.createByteArray() ?: ByteArray(0),
            nonce = parcel.createByteArray() ?: ByteArray(0),
            expiresAt = parcel.readLong(),
        )

    override fun ZomeCallUnsignedFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeByteArray(provenance)
        CellIdFfiParcel(cellId).writeToParcel(parcel, flags)
        parcel.writeString(zomeName)
        parcel.writeString(fnName)

        if (capSecret != null) {
            parcel.writeInt(1)
            parcel.writeByteArray(capSecret)
        } else {
            parcel.writeInt(0)
        }

        parcel.writeByteArray(payload)
        parcel.writeByteArray(nonce)
        parcel.writeLong(expiresAt)
    }
}

object ZomeCallFfiParceler : Parceler<ZomeCallFfi> {
    override fun create(parcel: Parcel): ZomeCallFfi =
        ZomeCallFfi(
            cellId = parcelableCreator<CellIdFfiParcel>().createFromParcel(parcel).inner,
            zomeName = parcel.readString() ?: "",
            fnName = parcel.readString() ?: "",
            payload = parcel.createByteArray() ?: ByteArray(0),
            capSecret = if (parcel.readInt() == 1) parcel.createByteArray() else null,
            provenance = parcel.createByteArray() ?: ByteArray(0),
            signature = parcel.createByteArray() ?: ByteArray(0),
            nonce = parcel.createByteArray() ?: ByteArray(0),
            expiresAt = parcel.readLong(),
        )

    override fun ZomeCallFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        CellIdFfiParcel(cellId).writeToParcel(parcel, flags)
        parcel.writeString(zomeName)
        parcel.writeString(fnName)
        parcel.writeByteArray(payload)

        if (capSecret != null) {
            parcel.writeInt(1)
            parcel.writeByteArray(capSecret)
        } else {
            parcel.writeInt(0)
        }

        parcel.writeByteArray(provenance)
        parcel.writeByteArray(signature)
        parcel.writeByteArray(nonce)
        parcel.writeLong(expiresAt)
    }
}

object CellIdFfiParceler : Parceler<CellIdFfi> {
    override fun create(parcel: Parcel): CellIdFfi =
        CellIdFfi(
            dnaHash = parcel.createByteArray() ?: ByteArray(0),
            agentPubKey = parcel.createByteArray() ?: ByteArray(0),
        )

    override fun CellIdFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeByteArray(dnaHash)
        parcel.writeByteArray(agentPubKey)
    }
}

object CellInfoFfiParceler : Parceler<CellInfoFfi> {
    override fun create(parcel: Parcel): CellInfoFfi {
        val i = parcel.readInt()
        return when (i) {
            1 ->
                CellInfoFfi.Provisioned(
                    parcelableCreator<ProvisionedCellFfiParcel>().createFromParcel(parcel).inner,
                )
            2 ->
                CellInfoFfi.Cloned(
                    parcelableCreator<ClonedCellFfiParcel>().createFromParcel(parcel).inner,
                )
            3 -> CellInfoFfi.Stem(parcelableCreator<StemCellFfiParcel>().createFromParcel(parcel).inner)
            else -> throw IllegalArgumentException("Unknown CellInfoFfi type")
        }
    }

    override fun CellInfoFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        when (this) {
            is CellInfoFfi.Provisioned -> {
                parcel.writeInt(1)
                ProvisionedCellFfiParcel(v1).writeToParcel(parcel, flags)
            }
            is CellInfoFfi.Cloned -> {
                parcel.writeInt(2)
                ClonedCellFfiParcel(v1).writeToParcel(parcel, flags)
            }
            is CellInfoFfi.Stem -> {
                parcel.writeInt(3)
                StemCellFfiParcel(v1).writeToParcel(parcel, flags)
            }
            else -> throw IllegalArgumentException("Unknown CellInfoFfi type")
        }
    }
}

object ProvisionedCellFfiParceler : Parceler<ProvisionedCellFfi> {
    override fun create(parcel: Parcel): ProvisionedCellFfi =
        ProvisionedCellFfi(
            cellId = parcelableCreator<CellIdFfiParcel>().createFromParcel(parcel).inner,
            dnaModifiers = parcelableCreator<DnaModifiersFfiParcel>().createFromParcel(parcel).inner,
            name = parcel.readString() ?: "",
        )

    override fun ProvisionedCellFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        CellIdFfiParcel(cellId).writeToParcel(parcel, flags)
        DnaModifiersFfiParcel(dnaModifiers).writeToParcel(parcel, flags)
        parcel.writeString(name)
    }
}

object ClonedCellFfiParceler : Parceler<ClonedCellFfi> {
    override fun create(parcel: Parcel): ClonedCellFfi =
        ClonedCellFfi(
            cellId = parcelableCreator<CellIdFfiParcel>().createFromParcel(parcel).inner,
            cloneId = parcel.readString() ?: "",
            originalDnaHash = parcel.createByteArray() ?: ByteArray(0),
            dnaModifiers = parcelableCreator<DnaModifiersFfiParcel>().createFromParcel(parcel).inner,
            name = parcel.readString() ?: "",
            enabled = parcel.readInt() == 1,
        )

    override fun ClonedCellFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        CellIdFfiParcel(cellId).writeToParcel(parcel, flags)
        parcel.writeString(cloneId)
        parcel.writeByteArray(originalDnaHash)
        DnaModifiersFfiParcel(dnaModifiers).writeToParcel(parcel, flags)
        parcel.writeString(name)
        parcel.writeInt(if (enabled) 1 else 0)
    }
}

object StemCellFfiParceler : Parceler<StemCellFfi> {
    override fun create(parcel: Parcel): StemCellFfi =
        StemCellFfi(
            originalDnaHash = parcel.createByteArray() ?: ByteArray(0),
            dnaModifiers = parcelableCreator<DnaModifiersFfiParcel>().createFromParcel(parcel).inner,
            name = if (parcel.readInt() == 1) parcel.readString() else null,
        )

    override fun StemCellFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeByteArray(originalDnaHash)
        DnaModifiersFfiParcel(dnaModifiers).writeToParcel(parcel, flags)
        if (name != null) {
            parcel.writeInt(1)
            parcel.writeString(name)
        } else {
            parcel.writeInt(0)
        }
    }
}

object DnaModifiersFfiParceler : Parceler<DnaModifiersFfi> {
    override fun create(parcel: Parcel): DnaModifiersFfi =
        DnaModifiersFfi(
            networkSeed = parcel.readString() ?: "",
            properties = parcel.createByteArray() ?: ByteArray(0),
            originTime = parcel.readLong(),
            quantumTime = parcelableCreator<DurationFfiParcel>().createFromParcel(parcel).inner,
        )

    override fun DnaModifiersFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeString(networkSeed)
        parcel.writeByteArray(properties)
        parcel.writeLong(originTime)
        DurationFfiParcel(quantumTime).writeToParcel(parcel, flags)
    }
}

object DnaModifiersOptFfiParceler : Parceler<DnaModifiersOptFfi> {
    override fun create(parcel: Parcel): DnaModifiersOptFfi =
        DnaModifiersOptFfi(
            networkSeed = parcel.readString() ?: "",
            properties = parcel.createByteArray() ?: ByteArray(0),
            originTime = parcel.readLong(),
            quantumTime = parcelableCreator<DurationFfiParcel>().createFromParcel(parcel).inner,
        )

    override fun DnaModifiersOptFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeString(networkSeed)
        parcel.writeByteArray(properties)

        if (originTime != null) {
            parcel.writeLong(originTime!!)
        }
        if (quantumTime != null) {
            DurationFfiParcel(quantumTime!!).writeToParcel(parcel, flags)
        }
    }
}

object DurationFfiParceler : Parceler<DurationFfi> {
    override fun create(parcel: Parcel): DurationFfi = DurationFfi(secs = parcel.readLong().toULong(), nanos = parcel.readInt().toUInt())

    override fun DurationFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeLong(secs.toLong())
        parcel.writeInt(nanos.toInt())
    }
}

object PausedAppReasonFfiParceler : Parceler<PausedAppReasonFfi> {
    override fun create(parcel: Parcel): PausedAppReasonFfi =
        when (parcel.readInt()) {
            1 -> PausedAppReasonFfi.Error(parcel.readString() ?: "")
            else -> throw IllegalArgumentException("Unknown PausedAppReasonFfi type")
        }

    override fun PausedAppReasonFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        when (this) {
            is PausedAppReasonFfi.Error -> {
                parcel.writeInt(1)
                parcel.writeString(v1)
            }
        }
    }
}

object DisabledAppReasonFfiParceler : Parceler<DisabledAppReasonFfi> {
    override fun create(parcel: Parcel): DisabledAppReasonFfi =
        when (parcel.readInt()) {
            1 -> DisabledAppReasonFfi.NeverStarted
            2 -> DisabledAppReasonFfi.NotStartedAfterProvidingMemproofs
            3 -> DisabledAppReasonFfi.DeletingAgentKey
            4 -> DisabledAppReasonFfi.User
            5 -> DisabledAppReasonFfi.Error(parcel.readString() ?: "")
            else -> throw IllegalArgumentException("Unknown DisabledAppReasonFfi type")
        }

    override fun DisabledAppReasonFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        when (this) {
            is DisabledAppReasonFfi.NeverStarted -> parcel.writeInt(1)
            is DisabledAppReasonFfi.NotStartedAfterProvidingMemproofs -> parcel.writeInt(2)
            is DisabledAppReasonFfi.DeletingAgentKey -> parcel.writeInt(3)
            is DisabledAppReasonFfi.User -> parcel.writeInt(4)
            is DisabledAppReasonFfi.Error -> {
                parcel.writeInt(5)
                parcel.writeString(v1)
            }
        }
    }
}

object AdminBinderUnauthorizedExceptionParceler : Parceler<AdminBinderUnauthorizedException> {
    override fun create(parcel: Parcel): AdminBinderUnauthorizedException = AdminBinderUnauthorizedException(parcel.readString()!!)

    override fun AdminBinderUnauthorizedException.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeString(message)
    }
}

object AppBinderUnauthorizedExceptionParceler : Parceler<AppBinderUnauthorizedException> {
    override fun create(parcel: Parcel): AppBinderUnauthorizedException = AppBinderUnauthorizedException(parcel.readString()!!)

    override fun AppBinderUnauthorizedException.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeString(message)
    }
}

object RuntimeNetworkConfigFfiParceler : Parceler<RuntimeNetworkConfigFfi> {
    override fun create(parcel: Parcel): RuntimeNetworkConfigFfi {
        val bootstrapUrl = parcel.readString() ?: ""
        val signalUrl = parcel.readString() ?: ""
        var iceUrls = mutableListOf<String>()
        parcel.readStringList(iceUrls)

        return RuntimeNetworkConfigFfi(
            bootstrapUrl = bootstrapUrl,
            signalUrl = signalUrl,
            iceUrls = iceUrls,
        )
    }

    override fun RuntimeNetworkConfigFfi.write(
        parcel: Parcel,
        flags: Int,
    ) {
        parcel.writeString(bootstrapUrl)
        parcel.writeString(signalUrl)
        parcel.writeStringList(iceUrls)
    }
}
