package org.holochain.androidserviceruntime.holochain_service_client

import androidx.test.ext.junit.runners.AndroidJUnit4
import java.nio.ByteBuffer
import java.util.UUID
import kotlin.random.Random
import org.junit.Assert.*
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class ParcelableTypesTest {

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
                rolesSettings = null,
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
}
