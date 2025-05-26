import android.os.Parcel
import android.os.Parcelable.PARCELABLE_WRITE_RETURN_VALUE
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.parcelize.parcelableCreator
import org.holochain.androidserviceruntime.client.AppAuthFfi
import org.holochain.androidserviceruntime.client.AppAuthFfiParcel
import org.holochain.androidserviceruntime.client.AppAuthenticationTokenIssuedFfi
import org.holochain.androidserviceruntime.client.AppAuthenticationTokenIssuedFfiParcel
import org.holochain.androidserviceruntime.client.AppInfoFfi
import org.holochain.androidserviceruntime.client.AppInfoFfiParcel
import org.holochain.androidserviceruntime.client.AppInfoStatusFfi
import org.holochain.androidserviceruntime.client.AppInfoStatusFfiParcel
import org.holochain.androidserviceruntime.client.CellIdFfi
import org.holochain.androidserviceruntime.client.CellIdFfiParcel
import org.holochain.androidserviceruntime.client.CellInfoFfi
import org.holochain.androidserviceruntime.client.CellInfoFfiParcel
import org.holochain.androidserviceruntime.client.ClonedCellFfi
import org.holochain.androidserviceruntime.client.ClonedCellFfiParcel
import org.holochain.androidserviceruntime.client.DisabledAppReasonFfi
import org.holochain.androidserviceruntime.client.DisabledAppReasonFfiParcel
import org.holochain.androidserviceruntime.client.DnaModifiersFfi
import org.holochain.androidserviceruntime.client.DnaModifiersFfiParcel
import org.holochain.androidserviceruntime.client.DnaModifiersOptFfi
import org.holochain.androidserviceruntime.client.DurationFfi
import org.holochain.androidserviceruntime.client.DurationFfiParcel
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfi
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfiParcel
import org.holochain.androidserviceruntime.client.PausedAppReasonFfi
import org.holochain.androidserviceruntime.client.PausedAppReasonFfiParcel
import org.holochain.androidserviceruntime.client.ProvisionedCellFfi
import org.holochain.androidserviceruntime.client.ProvisionedCellFfiParcel
import org.holochain.androidserviceruntime.client.RoleSettingsFfi
import org.holochain.androidserviceruntime.client.RuntimeNetworkConfigFfi
import org.holochain.androidserviceruntime.client.RuntimeNetworkConfigFfiParcel
import org.holochain.androidserviceruntime.client.StemCellFfi
import org.holochain.androidserviceruntime.client.StemCellFfiParcel
import org.holochain.androidserviceruntime.client.ZomeCallFfi
import org.holochain.androidserviceruntime.client.ZomeCallFfiParcel
import org.holochain.androidserviceruntime.client.ZomeCallUnsignedFfi
import org.holochain.androidserviceruntime.client.ZomeCallUnsignedFfiParcel
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import java.util.UUID
import kotlin.random.Random

@RunWith(AndroidJUnit4::class)
class ParcelablesTest {
    @Test
    fun testCellIdFfiParcel() {
        val value =
            CellIdFfiParcel(
                CellIdFfi(
                    dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                    agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<CellIdFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.dnaHash, readValue.inner.dnaHash)
        assertArrayEquals(value.inner.agentPubKey, readValue.inner.agentPubKey)
    }

    @Test
    fun testDurationFfiParcel() {
        val value = DurationFfiParcel(DurationFfi(secs = 100UL, nanos = 500U))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<DurationFfiParcel>().createFromParcel(parcel)

        assertEquals(value.inner.secs, readValue.inner.secs)
        assertEquals(value.inner.nanos, readValue.inner.nanos)
    }

    @Test
    fun testDnaModifiersFfiParcel() {
        val value =
            DnaModifiersFfiParcel(
                DnaModifiersFfi(
                    networkSeed = "1234",
                    properties = ByteArray(300) { Random.nextInt(256).toByte() },
                    originTime = 1000L,
                    quantumTime = DurationFfi(100UL, 100U),
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<DnaModifiersFfiParcel>().createFromParcel(parcel)

        assertEquals(value.inner.networkSeed, readValue.inner.networkSeed)
        assertArrayEquals(value.inner.properties, readValue.inner.properties)
        assertEquals(value.inner.originTime, readValue.inner.originTime)
        assertEquals(value.inner.quantumTime.secs, readValue.inner.quantumTime.secs)
        assertEquals(value.inner.quantumTime.nanos, readValue.inner.quantumTime.nanos)
    }

    @Test
    fun testCellInfoFfiParcel() {
        val value =
            CellInfoFfiParcel(
                CellInfoFfi.Provisioned(
                    ProvisionedCellFfi(
                        cellId =
                            CellIdFfi(
                                dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                                agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                            ),
                        dnaModifiers =
                            DnaModifiersFfi(
                                networkSeed = "1234",
                                properties = ByteArray(300) { Random.nextInt(256).toByte() },
                                originTime = 1000L,
                                quantumTime = DurationFfi(100UL, 100U),
                            ),
                        name = "cell-1",
                    ),
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<CellInfoFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(
            (value.inner as CellInfoFfi.Provisioned).v1.cellId.dnaHash,
            (readValue.inner as CellInfoFfi.Provisioned).v1.cellId.dnaHash,
        )
        assertArrayEquals(value.inner.v1.cellId.agentPubKey, readValue.inner.v1.cellId.agentPubKey)
        assertEquals(
            value.inner.v1.dnaModifiers.networkSeed,
            readValue.inner.v1.dnaModifiers.networkSeed,
        )
        assertArrayEquals(
            value.inner.v1.dnaModifiers.properties,
            readValue.inner.v1.dnaModifiers.properties,
        )
        assertEquals(value.inner.v1.dnaModifiers.originTime, readValue.inner.v1.dnaModifiers.originTime)
        assertEquals(
            value.inner.v1.dnaModifiers.quantumTime.secs,
            readValue.inner.v1.dnaModifiers.quantumTime.secs,
        )
        assertEquals(
            value.inner.v1.dnaModifiers.quantumTime.nanos,
            readValue.inner.v1.dnaModifiers.quantumTime.nanos,
        )
        assertEquals(value.inner.v1.name, readValue.inner.v1.name)
    }

    @Test
    fun testAppInfoFfiParcel() {
        val value =
            AppInfoFfiParcel(
                AppInfoFfi(
                    installedAppId = "my-app",
                    cellInfo =
                        mapOf(
                            "cell-1" to
                                listOf(
                                    CellInfoFfi.Provisioned(
                                        ProvisionedCellFfi(
                                            cellId =
                                                CellIdFfi(
                                                    dnaHash =
                                                        ByteArray(32) { Random.nextInt(256).toByte() },
                                                    agentPubKey =
                                                        ByteArray(32) { Random.nextInt(256).toByte() },
                                                ),
                                            dnaModifiers =
                                                DnaModifiersFfi(
                                                    networkSeed = "1234",
                                                    properties =
                                                        ByteArray(300) { Random.nextInt(256).toByte() },
                                                    originTime = 1000L,
                                                    quantumTime = DurationFfi(100UL, 100U),
                                                ),
                                            name = "cell-1",
                                        ),
                                    ),
                                ),
                        ),
                    status = AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.NeverStarted),
                    agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppInfoFfiParcel>().createFromParcel(parcel)

        assertEquals(value.inner.installedAppId, readValue.inner.installedAppId)
        assertEquals(readValue.inner.cellInfo.keys.size, 1)
        assertEquals(
            readValue.inner.cellInfo.keys
                .first(),
            "cell-1",
        )
        assertEquals(readValue.inner.cellInfo.values.size, 1)

        assert(readValue.inner.cellInfo["cell-1"]!![0] is CellInfoFfi.Provisioned)
        assertArrayEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned).v1.cellId.dnaHash,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned).v1.cellId.dnaHash,
        )
        assertArrayEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned).v1.cellId.agentPubKey,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned).v1.cellId.agentPubKey,
        )
        assertEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .networkSeed,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .networkSeed,
        )
        assertArrayEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned).v1.dnaModifiers.properties,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .properties,
        )
        assertEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned).v1.dnaModifiers.originTime,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .originTime,
        )
        assertEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .quantumTime
                .secs,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .quantumTime
                .secs,
        )
        assertEquals(
            (value.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .quantumTime
                .nanos,
            (readValue.inner.cellInfo["cell-1"]!![0] as CellInfoFfi.Provisioned)
                .v1
                .dnaModifiers
                .quantumTime
                .nanos,
        )
        assert(
            (readValue.inner.status as AppInfoStatusFfi.Disabled).reason
                is DisabledAppReasonFfi.NeverStarted,
        )
        assertArrayEquals(value.inner.agentPubKey, readValue.inner.agentPubKey)
    }

    @Test
    fun testInstallAppPayloadFfiParcel() {
        val value =
            InstallAppPayloadFfiParcel(
                InstallAppPayloadFfi(
                    source = ByteArray(5000) { Random.nextInt(256).toByte() },
                    installedAppId = "my-app",
                    networkSeed = UUID.randomUUID().toString(),
                    rolesSettings =
                        mapOf(
                            "cell-1" to
                                RoleSettingsFfi.Provisioned(
                                    membraneProof = ByteArray(60) { Random.nextInt(256).toByte() },
                                    modifiers =
                                        DnaModifiersOptFfi(
                                            networkSeed = "1234",
                                            properties =
                                                ByteArray(300) { Random.nextInt(256).toByte() },
                                            originTime = 1000L,
                                            quantumTime = DurationFfi(100UL, 100U),
                                        ),
                                ),
                        ),
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<InstallAppPayloadFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.source, readValue.inner.source)
        assertEquals(value.inner.installedAppId, readValue.inner.installedAppId)
        assertEquals(value.inner.networkSeed, readValue.inner.networkSeed)
        assertEquals(
            value.inner.rolesSettings!!
                .keys.size,
            1,
        )
        assertEquals(
            value.inner.rolesSettings!!
                .keys
                .first(),
            readValue.inner.rolesSettings!!
                .keys
                .first(),
        )
        assert(value.inner.rolesSettings!!.get("cell-1")!! is RoleSettingsFfi.Provisioned)
        assertArrayEquals(
            (value.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned).membraneProof,
            (readValue.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .membraneProof,
        )
        assertEquals(
            (value.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .networkSeed,
            (readValue.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .networkSeed,
        )
        assertArrayEquals(
            (value.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .properties,
            (readValue.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .properties,
        )
        assertEquals(
            (value.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .originTime,
            (readValue.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .originTime,
        )
        assertEquals(
            (value.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .quantumTime!!
                .nanos,
            (readValue.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .quantumTime!!
                .nanos,
        )
        assertEquals(
            (value.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .quantumTime!!
                .secs,
            (readValue.inner.rolesSettings!!.get("cell-1")!! as RoleSettingsFfi.Provisioned)
                .modifiers!!
                .quantumTime!!
                .secs,
        )
    }

    @Test
    fun testInstallAppPayloadFfiParcelNulls() {
        val value =
            InstallAppPayloadFfiParcel(
                InstallAppPayloadFfi(
                    source = ByteArray(5000) { Random.nextInt(256).toByte() },
                    installedAppId = "app-id",
                    networkSeed = null,
                    rolesSettings = null,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<InstallAppPayloadFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.source, readValue.inner.source)
        assertEquals(value.inner.installedAppId, readValue.inner.installedAppId)
        assertEquals(value.inner.networkSeed, readValue.inner.networkSeed)
        assertEquals(value.inner.rolesSettings, readValue.inner.rolesSettings)
    }

    @Test
    fun testPausedAppReasonFfiParcel() {
        val value = PausedAppReasonFfiParcel(PausedAppReasonFfi.Error("my error"))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<PausedAppReasonFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is PausedAppReasonFfi.Error)
        assertEquals((readValue.inner as PausedAppReasonFfi.Error).v1, "my error")
    }

    @Test
    fun testDisabledAppReasonFfiParcelNeverStarted() {
        val value = DisabledAppReasonFfiParcel(DisabledAppReasonFfi.NeverStarted)

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<DisabledAppReasonFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is DisabledAppReasonFfi.NeverStarted)
    }

    @Test
    fun testDisabledAppReasonFfiParcelError() {
        val value = DisabledAppReasonFfiParcel(DisabledAppReasonFfi.Error("my error"))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<DisabledAppReasonFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is DisabledAppReasonFfi.Error)
        assertEquals((readValue.inner as DisabledAppReasonFfi.Error).v1, "my error")
    }

    @Test
    fun testAppInfoStatusFfiParcelRunning() {
        val value = AppInfoStatusFfiParcel(AppInfoStatusFfi.Running)

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppInfoStatusFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is AppInfoStatusFfi.Running)
    }

    @Test
    fun testAppInfoStatusFfiParcelAwaitingMemproofs() {
        val value = AppInfoStatusFfiParcel(AppInfoStatusFfi.AwaitingMemproofs)

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppInfoStatusFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is AppInfoStatusFfi.AwaitingMemproofs)
    }

    @Test
    fun testAppInfoStatusFfiParcelPaused() {
        val value =
            AppInfoStatusFfiParcel(AppInfoStatusFfi.Paused(PausedAppReasonFfi.Error("my error")))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppInfoStatusFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is AppInfoStatusFfi.Paused)
        assert((readValue.inner as AppInfoStatusFfi.Paused).reason is PausedAppReasonFfi.Error)
        assertEquals((readValue.inner.reason as PausedAppReasonFfi.Error).v1, "my error")
    }

    @Test
    fun testAppInfoStatusFfiParcelDisabledNeverStarted() {
        val value = AppInfoStatusFfiParcel(AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.NeverStarted))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppInfoStatusFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is AppInfoStatusFfi.Disabled)
        assert(
            (readValue.inner as AppInfoStatusFfi.Disabled).reason is DisabledAppReasonFfi.NeverStarted,
        )
    }

    @Test
    fun testAppInfoStatusFfiParcelDisabledError() {
        val value =
            AppInfoStatusFfiParcel(AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.Error("my error")))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppInfoStatusFfiParcel>().createFromParcel(parcel)

        assert(readValue.inner is AppInfoStatusFfi.Disabled)
        assert((readValue.inner as AppInfoStatusFfi.Disabled).reason is DisabledAppReasonFfi.Error)
        assertEquals((readValue.inner.reason as DisabledAppReasonFfi.Error).v1, "my error")
    }

    @Test
    fun testAppAuthenticationTokenIssuedFfiParcel() {
        val value =
            AppAuthenticationTokenIssuedFfiParcel(
                AppAuthenticationTokenIssuedFfi(
                    token = ByteArray(50) { Random.nextInt(256).toByte() },
                    expiresAt = 1000000L,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue =
            parcelableCreator<AppAuthenticationTokenIssuedFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.token, readValue.inner.token)
        assertEquals(value.inner.expiresAt, readValue.inner.expiresAt)
    }

    @Test
    fun testAppAuthenticationTokenIssuedFfiParcelNulls() {
        val value =
            AppAuthenticationTokenIssuedFfiParcel(
                AppAuthenticationTokenIssuedFfi(
                    token = ByteArray(50) { Random.nextInt(256).toByte() },
                    expiresAt = null,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue =
            parcelableCreator<AppAuthenticationTokenIssuedFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.token, readValue.inner.token)
        assertEquals(value.inner.expiresAt, readValue.inner.expiresAt)
    }

    @Test
    fun testAppAuthFfiParcel() {
        val value =
            AppAuthFfiParcel(
                AppAuthFfi(
                    authentication =
                        AppAuthenticationTokenIssuedFfi(
                            token = ByteArray(50) { Random.nextInt(256).toByte() },
                            expiresAt = 1500000L,
                        ),
                    port = 4200U,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<AppAuthFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.authentication.token, readValue.inner.authentication.token)
        assertEquals(value.inner.authentication.expiresAt, readValue.inner.authentication.expiresAt)
        assertEquals(value.inner.port, readValue.inner.port)
    }

    @Test
    fun testZomeCallUnsignedFfiParcel() {
        val value =
            ZomeCallUnsignedFfiParcel(
                ZomeCallUnsignedFfi(
                    provenance = ByteArray(32) { Random.nextInt(256).toByte() },
                    cellId =
                        CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                    zomeName = "my_zome",
                    fnName = "my_function",
                    capSecret = ByteArray(50) { Random.nextInt(256).toByte() },
                    payload = ByteArray(50) { Random.nextInt(256).toByte() },
                    nonce = ByteArray(50) { Random.nextInt(256).toByte() },
                    expiresAt = 10000000L,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<ZomeCallUnsignedFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.provenance, readValue.inner.provenance)
        assertArrayEquals(value.inner.cellId.dnaHash, readValue.inner.cellId.dnaHash)
        assertArrayEquals(value.inner.cellId.agentPubKey, readValue.inner.cellId.agentPubKey)
        assertEquals(value.inner.zomeName, readValue.inner.zomeName)
        assertEquals(value.inner.fnName, readValue.inner.fnName)
        assertArrayEquals(value.inner.capSecret, readValue.inner.capSecret)
        assertArrayEquals(value.inner.payload, readValue.inner.payload)
        assertArrayEquals(value.inner.nonce, readValue.inner.nonce)
        assertEquals(value.inner.expiresAt, readValue.inner.expiresAt)
    }

    @Test
    fun testZomeCallUnsignedFfiParcelNulls() {
        val value =
            ZomeCallUnsignedFfiParcel(
                ZomeCallUnsignedFfi(
                    provenance = ByteArray(32) { Random.nextInt(256).toByte() },
                    cellId =
                        CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                    zomeName = "my_zome",
                    fnName = "my_function",
                    capSecret = null,
                    payload = ByteArray(50) { Random.nextInt(256).toByte() },
                    nonce = ByteArray(50) { Random.nextInt(256).toByte() },
                    expiresAt = 10000000L,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<ZomeCallUnsignedFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.provenance, readValue.inner.provenance)
        assertArrayEquals(value.inner.cellId.dnaHash, readValue.inner.cellId.dnaHash)
        assertArrayEquals(value.inner.cellId.agentPubKey, readValue.inner.cellId.agentPubKey)
        assertEquals(value.inner.zomeName, readValue.inner.zomeName)
        assertEquals(value.inner.fnName, readValue.inner.fnName)
        assertEquals(value.inner.capSecret, readValue.inner.capSecret)
        assertArrayEquals(value.inner.payload, readValue.inner.payload)
        assertArrayEquals(value.inner.nonce, readValue.inner.nonce)
        assertEquals(value.inner.expiresAt, readValue.inner.expiresAt)
    }

    @Test
    fun testZomeCallFfiParcel() {
        val value =
            ZomeCallFfiParcel(
                ZomeCallFfi(
                    provenance = ByteArray(32) { Random.nextInt(256).toByte() },
                    cellId =
                        CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                    zomeName = "my_zome",
                    fnName = "my_function",
                    capSecret = ByteArray(50) { Random.nextInt(256).toByte() },
                    payload = ByteArray(50) { Random.nextInt(256).toByte() },
                    nonce = ByteArray(50) { Random.nextInt(256).toByte() },
                    expiresAt = 10000000L,
                    signature = ByteArray(50) { Random.nextInt(256).toByte() },
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<ZomeCallFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.provenance, readValue.inner.provenance)
        assertArrayEquals(value.inner.cellId.dnaHash, readValue.inner.cellId.dnaHash)
        assertArrayEquals(value.inner.cellId.agentPubKey, readValue.inner.cellId.agentPubKey)
        assertEquals(value.inner.zomeName, readValue.inner.zomeName)
        assertEquals(value.inner.fnName, readValue.inner.fnName)
        assertArrayEquals(value.inner.capSecret, readValue.inner.capSecret)
        assertArrayEquals(value.inner.payload, readValue.inner.payload)
        assertArrayEquals(value.inner.nonce, readValue.inner.nonce)
        assertEquals(value.inner.expiresAt, readValue.inner.expiresAt)
        assertArrayEquals(value.inner.signature, readValue.inner.signature)
    }

    @Test
    fun testZomeCallFfiParcelNulls() {
        val value =
            ZomeCallFfiParcel(
                ZomeCallFfi(
                    provenance = ByteArray(32) { Random.nextInt(256).toByte() },
                    cellId =
                        CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                    zomeName = "my_zome",
                    fnName = "my_function",
                    capSecret = null,
                    payload = ByteArray(50) { Random.nextInt(256).toByte() },
                    nonce = ByteArray(50) { Random.nextInt(256).toByte() },
                    expiresAt = 10000000L,
                    signature = ByteArray(50) { Random.nextInt(256).toByte() },
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<ZomeCallFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.provenance, readValue.inner.provenance)
        assertArrayEquals(value.inner.cellId.dnaHash, readValue.inner.cellId.dnaHash)
        assertArrayEquals(value.inner.cellId.agentPubKey, readValue.inner.cellId.agentPubKey)
        assertEquals(value.inner.zomeName, readValue.inner.zomeName)
        assertEquals(value.inner.fnName, readValue.inner.fnName)
        assertEquals(value.inner.capSecret, readValue.inner.capSecret)
        assertArrayEquals(value.inner.payload, readValue.inner.payload)
        assertArrayEquals(value.inner.nonce, readValue.inner.nonce)
        assertEquals(value.inner.expiresAt, readValue.inner.expiresAt)
        assertArrayEquals(value.inner.signature, readValue.inner.signature)
    }

    @Test
    fun testProvisionedCellFfiParcel() {
        val value =
            ProvisionedCellFfiParcel(
                ProvisionedCellFfi(
                    cellId =
                        CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                    dnaModifiers =
                        DnaModifiersFfi(
                            networkSeed = "1234",
                            properties = ByteArray(300) { Random.nextInt(256).toByte() },
                            originTime = 1000L,
                            quantumTime = DurationFfi(100UL, 100U),
                        ),
                    name = "cell-1",
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<ProvisionedCellFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.cellId.dnaHash, readValue.inner.cellId.dnaHash)
        assertArrayEquals(value.inner.cellId.agentPubKey, readValue.inner.cellId.agentPubKey)
        assertEquals(value.inner.dnaModifiers.networkSeed, readValue.inner.dnaModifiers.networkSeed)
        assertArrayEquals(value.inner.dnaModifiers.properties, readValue.inner.dnaModifiers.properties)
        assertEquals(value.inner.dnaModifiers.originTime, readValue.inner.dnaModifiers.originTime)
        assertEquals(
            value.inner.dnaModifiers.quantumTime.secs,
            readValue.inner.dnaModifiers.quantumTime.secs,
        )
        assertEquals(
            value.inner.dnaModifiers.quantumTime.nanos,
            readValue.inner.dnaModifiers.quantumTime.nanos,
        )
        assertEquals(value.inner.name, readValue.inner.name)
    }

    @Test
    fun testClonedCellFfiParcel() {
        val value =
            ClonedCellFfiParcel(
                ClonedCellFfi(
                    cellId =
                        CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                    cloneId = "cell-1.1",
                    originalDnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                    dnaModifiers =
                        DnaModifiersFfi(
                            networkSeed = "1234",
                            properties = ByteArray(300) { Random.nextInt(256).toByte() },
                            originTime = 1000L,
                            quantumTime = DurationFfi(100UL, 100U),
                        ),
                    name = "cell-1",
                    enabled = true,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<ClonedCellFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.cellId.dnaHash, readValue.inner.cellId.dnaHash)
        assertArrayEquals(value.inner.cellId.agentPubKey, readValue.inner.cellId.agentPubKey)
        assertEquals(value.inner.cloneId, readValue.inner.cloneId)
        assertArrayEquals(value.inner.originalDnaHash, readValue.inner.originalDnaHash)
        assertEquals(value.inner.dnaModifiers.networkSeed, readValue.inner.dnaModifiers.networkSeed)
        assertArrayEquals(value.inner.dnaModifiers.properties, readValue.inner.dnaModifiers.properties)
        assertEquals(value.inner.dnaModifiers.originTime, readValue.inner.dnaModifiers.originTime)
        assertEquals(
            value.inner.dnaModifiers.quantumTime.secs,
            readValue.inner.dnaModifiers.quantumTime.secs,
        )
        assertEquals(
            value.inner.dnaModifiers.quantumTime.nanos,
            readValue.inner.dnaModifiers.quantumTime.nanos,
        )
        assertEquals(value.inner.name, readValue.inner.name)
        assertEquals(value.inner.enabled, readValue.inner.enabled)
    }

    @Test
    fun testStemCellFfiParcel() {
        val value =
            StemCellFfiParcel(
                StemCellFfi(
                    originalDnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                    dnaModifiers =
                        DnaModifiersFfi(
                            networkSeed = "1234",
                            properties = ByteArray(300) { Random.nextInt(256).toByte() },
                            originTime = 1000L,
                            quantumTime = DurationFfi(100UL, 100U),
                        ),
                    name = "cell-1",
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<StemCellFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.originalDnaHash, readValue.inner.originalDnaHash)
        assertEquals(value.inner.dnaModifiers.networkSeed, readValue.inner.dnaModifiers.networkSeed)
        assertArrayEquals(value.inner.dnaModifiers.properties, readValue.inner.dnaModifiers.properties)
        assertEquals(value.inner.dnaModifiers.originTime, readValue.inner.dnaModifiers.originTime)
        assertEquals(
            value.inner.dnaModifiers.quantumTime.secs,
            readValue.inner.dnaModifiers.quantumTime.secs,
        )
        assertEquals(
            value.inner.dnaModifiers.quantumTime.nanos,
            readValue.inner.dnaModifiers.quantumTime.nanos,
        )
        assertEquals(value.inner.name, readValue.inner.name)
    }

    @Test
    fun testStemCellFfiParcelNulls() {
        val value =
            StemCellFfiParcel(
                StemCellFfi(
                    originalDnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                    dnaModifiers =
                        DnaModifiersFfi(
                            networkSeed = "1234",
                            properties = ByteArray(300) { Random.nextInt(256).toByte() },
                            originTime = 1000L,
                            quantumTime = DurationFfi(100UL, 100U),
                        ),
                    name = null,
                ),
            )

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<StemCellFfiParcel>().createFromParcel(parcel)

        assertArrayEquals(value.inner.originalDnaHash, readValue.inner.originalDnaHash)
        assertEquals(value.inner.dnaModifiers.networkSeed, readValue.inner.dnaModifiers.networkSeed)
        assertArrayEquals(value.inner.dnaModifiers.properties, readValue.inner.dnaModifiers.properties)
        assertEquals(value.inner.dnaModifiers.originTime, readValue.inner.dnaModifiers.originTime)
        assertEquals(
            value.inner.dnaModifiers.quantumTime.secs,
            readValue.inner.dnaModifiers.quantumTime.secs,
        )
        assertEquals(
            value.inner.dnaModifiers.quantumTime.nanos,
            readValue.inner.dnaModifiers.quantumTime.nanos,
        )
        assertEquals(value.inner.name, readValue.inner.name)
    }

    @Test
    fun testRuntimeNetworkConfigFfiParcel() {
        val value =
            RuntimeNetworkConfigFfiParcel(
                RuntimeNetworkConfigFfi(
                    bootstrapUrl = "2",
                    signalUrl = "3",
                    iceUrls = listOf("4", "5"),
                ),
            )
        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcelableCreator<RuntimeNetworkConfigFfiParcel>().createFromParcel(parcel)

        assertEquals(value.inner.bootstrapUrl, readValue.inner.bootstrapUrl)
        assertEquals(value.inner.signalUrl, readValue.inner.signalUrl)
        assertEquals(value.inner.iceUrls.size, readValue.inner.iceUrls.size)
        assertEquals(value.inner.iceUrls.first(), readValue.inner.iceUrls.first())
    }
}
