import androidx.test.ext.junit.runners.AndroidJUnit4
import java.nio.ByteBuffer
import java.util.UUID
import kotlin.random.Random
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith
import android.os.Parcel
import android.os.Parcelable.PARCELABLE_WRITE_RETURN_VALUE
import org.holochain.androidserviceruntime.holochain_service_client.*

@RunWith(AndroidJUnit4::class)
class ParcelablesTest {
    @Test
    fun testCellIdFfiParcel() {
        val value = CellIdFfiParcel(CellIdFfi(
            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
        ))

        val parcel = Parcel.obtain()
        parcel.writeParcelable(value, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<CellIdFfiParcel>(CellIdFfiParcel::class.java.classLoader,
            CellIdFfiParcel::class.java)!!

        assertArrayEquals(value.inner.dnaHash, readValue.inner.dnaHash)
        assertArrayEquals(value.inner.agentPubKey, readValue.inner.agentPubKey)
    }

    @Test
    fun testDurationFfiParcel() {
        val value = DurationFfiParcel(DurationFfi(
            secs = 100UL,
            nanos = 500U
        ))

        val parcel = Parcel.obtain()
        parcel.writeParcelable(value, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<DurationFfiParcel>(DurationFfiParcel::class.java.classLoader,
            DurationFfiParcel::class.java)!!

        assertEquals(value.inner.secs, readValue.inner.secs)
        assertEquals(value.inner.nanos, readValue.inner.nanos)
    }

    @Test
    fun testDnaModifiersFfiParcel() {
        val value = DnaModifiersFfiParcel(DnaModifiersFfi(
            networkSeed = "1234",
            properties = ByteArray(300) { Random.nextInt(256).toByte() },
            originTime = 1000L,
            quantumTime = DurationFfi(100UL, 100U)
        ))

        val parcel = Parcel.obtain()
        parcel.writeParcelable(value, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<DnaModifiersFfiParcel>(DnaModifiersFfiParcel::class.java.classLoader,
            DnaModifiersFfiParcel::class.java)!!

        assertEquals(value.inner.networkSeed, readValue.inner.networkSeed)
        assertArrayEquals(value.inner.properties, readValue.inner.properties)
        assertEquals(value.inner.originTime, readValue.inner.originTime)
        assertEquals(value.inner.quantumTime.secs, readValue.inner.quantumTime.secs)
        assertEquals(value.inner.quantumTime.nanos, readValue.inner.quantumTime.nanos)
    }

    @Test
    fun testCellInfoFfiParcel() {
        val value = CellInfoFfiParcel(CellInfoFfi.Provisioned(ProvisionedCellFfi(
            cellId = CellIdFfi(
                dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
            ),
            dnaModifiers = DnaModifiersFfi(
                networkSeed = "1234",
                properties = ByteArray(300) { Random.nextInt(256).toByte() },
                originTime = 1000L,
                quantumTime = DurationFfi(100UL, 100U)
            ),
            name = "cell-1"
        )))

        val parcel = Parcel.obtain()
        parcel.writeParcelable(value, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<CellInfoFfiParcel>(CellInfoFfiParcel::class.java.classLoader,
            CellInfoFfiParcel::class.java)!!

        assertArrayEquals((value.inner as CellInfoFfi.Provisioned).v1.cellId.dnaHash, (readValue.inner as CellInfoFfi.Provisioned).v1.cellId.dnaHash)
        assertArrayEquals(value.inner.v1.cellId.agentPubKey, readValue.inner.v1.cellId.agentPubKey)
        assertEquals(value.inner.v1.dnaModifiers.networkSeed, readValue.inner.v1.dnaModifiers.networkSeed)
        assertArrayEquals(value.inner.v1.dnaModifiers.properties, readValue.inner.v1.dnaModifiers.properties)
        assertEquals(value.inner.v1.dnaModifiers.originTime, readValue.inner.v1.dnaModifiers.originTime)
        assertEquals(value.inner.v1.dnaModifiers.quantumTime.secs, readValue.inner.v1.dnaModifiers.quantumTime.secs)
        assertEquals(value.inner.v1.dnaModifiers.quantumTime.nanos, readValue.inner.v1.dnaModifiers.quantumTime.nanos)
        assertEquals((value.inner as CellInfoFfi.Provisioned).v1.name, (readValue.inner as CellInfoFfi.Provisioned).v1.name)
    }

    @Test
    fun testAppInfoFfiParcel() {
        val value = AppInfoFfiParcel(AppInfoFfi(
            installedAppId = "my-app",
            cellInfo = mapOf(
                "cell-1" to listOf(
                    CellInfoFfi.Provisioned(ProvisionedCellFfi(
                        cellId = CellIdFfi(
                            dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
                        ),
                        dnaModifiers = DnaModifiersFfi(
                            networkSeed = "1234",
                            properties = ByteArray(300) { Random.nextInt(256).toByte() },
                            originTime = 1000L,
                            quantumTime = DurationFfi(100UL, 100U)
                        ),
                        name = "cell-1"
                    ))
                )
            ),
            status = AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.NeverStarted),
            agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
        ))

        val parcel = Parcel.obtain()
        parcel.writeParcelable(value, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<AppInfoFfiParcel>(AppInfoFfiParcel::class.java.classLoader, AppInfoFfiParcel::class.java)!!
            
        assertEquals(value.inner.installedAppId, readValue.inner.installedAppId)
    }

    @Test
    fun testByteBufferToByteArray() {
        val value = ByteArray(1000) { Random.nextInt(256).toByte() }
        val buffer = ByteBuffer.allocate(1000).put(value)

        // Convert to byte array
        val byteArray = buffer.toByteArray()
        assertArrayEquals(byteArray, value)
    }

    @Test
    fun testInstallAppPayloadFfiParcel() {
        val payload =
            InstallAppPayloadFfi(
                source = ByteArray(5000) { Random.nextInt(256).toByte() },
                installedAppId = "my-app",
                networkSeed = UUID.randomUUID().toString(),
                rolesSettings = mapOf("cell-1" to RoleSettingsFfi.Provisioned(
                    membraneProof = ByteArray(60) { Random.nextInt(256).toByte() },
                    modifiers = DnaModifiersOptFfi(
                        networkSeed = "1234",
                        properties = ByteArray(300) { Random.nextInt(256).toByte() },
                        originTime = 1000L,
                        quantumTime = DurationFfi(100UL, 100U)
                    ),
                )),
            )

        // Convert to parcel
        val parcelPayload = payload.toParcel()
        val readBuffer = parcelPayload.sourceSharedMemory.mapReadOnly()
        val retrievedSource = ByteArray(payload.source.size)
        readBuffer.get(retrievedSource)

        assertEquals(payload.installedAppId, parcelPayload.installedAppId)
        assertEquals(payload.networkSeed, parcelPayload.networkSeed)
        assertArrayEquals(payload.source, retrievedSource)

        // Convert from parcel
        val reconstructedPayload = parcelPayload.fromParcel()
        assertArrayEquals(payload.source, reconstructedPayload.source)
        assertEquals(payload.installedAppId, reconstructedPayload.installedAppId)
        assertEquals(payload.networkSeed, reconstructedPayload.networkSeed)
    }

    @Test
    fun testAppInfoStatusFfiParcel() {
        val value = AppInfoStatusFfiParcel(AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.NeverStarted))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<AppInfoStatusFfiParcel>(AppInfoStatusFfiParcel::class.java.classLoader, AppInfoStatusFfiParcel::class.java)!!
        parcel.recycle()
    }

    @Test
    fun testAppAuthenticationTokenIssuedFfiParcel() {
        val value = AppAuthenticationTokenIssuedFfiParcel(AppAuthenticationTokenIssuedFfi(
            token = ByteArray(50) { Random.nextInt(256).toByte() },
            expiresAt = 1000000L
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<AppAuthenticationTokenIssuedFfiParcel>(AppAuthenticationTokenIssuedFfiParcel::class.java.classLoader, AppAuthenticationTokenIssuedFfiParcel::class.java)!!
    }

    @Test
    fun testAppAuthFfiParcel() {
        val value = AppAuthFfiParcel(AppAuthFfi(
            authentication = AppAuthenticationTokenIssuedFfi(
                token = ByteArray(50) { Random.nextInt(256).toByte() },
                expiresAt = 1500000L
            ),
            port = 4200U
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<AppAuthFfiParcel>(AppAuthFfiParcel::class.java.classLoader, AppAuthFfiParcel::class.java)!!
    }

    @Test
    fun testZomeCallUnsignedFfiParcel() {
        val value = ZomeCallUnsignedFfiParcel(ZomeCallUnsignedFfi(
            provenance = ByteArray(32) { Random.nextInt(256).toByte() },
            cellId = CellIdFfi(
                dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
            ),
            zomeName = "my_zome",
            fnName =  "my_function",
            capSecret =  ByteArray(50) { Random.nextInt(256).toByte() },
            payload = ByteArray(50) { Random.nextInt(256).toByte() },
            nonce = ByteArray(50) { Random.nextInt(256).toByte() },
            expiresAt = 10000000L
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<ZomeCallUnsignedFfiParcel>(ZomeCallUnsignedFfiParcel::class.java.classLoader, ZomeCallUnsignedFfiParcel::class.java)!!
    }

    @Test
    fun testZomeCallFfiParcel() {
        val value = ZomeCallFfiParcel(ZomeCallFfi(
            provenance = ByteArray(32) { Random.nextInt(256).toByte() },
            cellId = CellIdFfi(
                dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
            ),
            zomeName = "my_zome",
            fnName =  "my_function",
            capSecret =  ByteArray(50) { Random.nextInt(256).toByte() },
            payload = ByteArray(50) { Random.nextInt(256).toByte() },
            nonce = ByteArray(50) { Random.nextInt(256).toByte() },
            expiresAt = 10000000L,
            signature = ByteArray(50) { Random.nextInt(256).toByte() },
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<ZomeCallFfiParcel>(ZomeCallFfiParcel::class.java.classLoader, ZomeCallFfiParcel::class.java)!!
    }


    @Test
    fun testPausedAppReasonFfiParcel() {
        val value = PausedAppReasonFfiParcel(PausedAppReasonFfi.Error(v1 = "my error"))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<PausedAppReasonFfiParcel>(PausedAppReasonFfiParcel::class.java.classLoader, PausedAppReasonFfiParcel::class.java)!!

        assert(value == readValue)
    }

    @Test
    fun testDisabledAppReasonFfiParcel() {
        val value = DisabledAppReasonFfiParcel(DisabledAppReasonFfi.NeverStarted)

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<DisabledAppReasonFfiParcel>(DisabledAppReasonFfiParcel::class.java.classLoader, DisabledAppReasonFfiParcel::class.java)!!
    }

    @Test
    fun testProvisionedCellFfiParcel() {
        val value = ProvisionedCellFfiParcel(ProvisionedCellFfi(
            cellId = CellIdFfi(
                dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
            ),
            dnaModifiers = DnaModifiersFfi(
                networkSeed = "1234",
                properties = ByteArray(300) { Random.nextInt(256).toByte() },
                originTime = 1000L,
                quantumTime = DurationFfi(100UL, 100U)
            ),
            name = "cell-1"
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<ProvisionedCellFfiParcel>(ProvisionedCellFfiParcel::class.java.classLoader, ProvisionedCellFfiParcel::class.java)!!
    }

    @Test
    fun testClonedCellFfiParcel() {
        val value = ClonedCellFfiParcel(ClonedCellFfi(
            cellId = CellIdFfi(
                dnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
                agentPubKey = ByteArray(32) { Random.nextInt(256).toByte() },
            ),
            cloneId = "cell-1.1",
            originalDnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
            dnaModifiers = DnaModifiersFfi(
                networkSeed = "1234",
                properties = ByteArray(300) { Random.nextInt(256).toByte() },
                originTime = 1000L,
                quantumTime = DurationFfi(100UL, 100U)
            ),
            name = "cell-1",
            enabled = true
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<ClonedCellFfiParcel>(ClonedCellFfiParcel::class.java.classLoader, ClonedCellFfiParcel::class.java)!!
    }

    @Test
    fun testStemCellFfiParcel() {
        val value = StemCellFfiParcel(StemCellFfi(
            originalDnaHash = ByteArray(32) { Random.nextInt(256).toByte() },
            dnaModifiers = DnaModifiersFfi(
                networkSeed = "1234",
                properties = ByteArray(300) { Random.nextInt(256).toByte() },
                originTime = 1000L,
                quantumTime = DurationFfi(100UL, 100U)
            ),
            name = "cell-1"
        ))

        val parcel = Parcel.obtain()
        value.writeToParcel(parcel, PARCELABLE_WRITE_RETURN_VALUE)
        parcel.setDataPosition(0)
        val readValue = parcel.readParcelable<StemCellFfiParcel>(StemCellFfiParcel::class.java.classLoader, StemCellFfiParcel::class.java)!!

        assert(value.inner == readValue.inner)
    }
}
