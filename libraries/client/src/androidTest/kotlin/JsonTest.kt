import androidx.test.ext.junit.runners.AndroidJUnit4
import org.holochain.androidserviceruntime.client.AppInfoStatusFfi
import org.holochain.androidserviceruntime.client.CellIdFfi
import org.holochain.androidserviceruntime.client.CellInfoFfi
import org.holochain.androidserviceruntime.client.DisabledAppReasonFfi
import org.holochain.androidserviceruntime.client.DnaModifiersFfi
import org.holochain.androidserviceruntime.client.DnaModifiersOptFfi
import org.holochain.androidserviceruntime.client.DurationFfi
import org.holochain.androidserviceruntime.client.PausedAppReasonFfi
import org.holochain.androidserviceruntime.client.ProvisionedCellFfi
import org.holochain.androidserviceruntime.client.RoleSettingsFfi
import org.holochain.androidserviceruntime.client.toJSONArray
import org.holochain.androidserviceruntime.client.toJSONObject
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test
import org.junit.runner.RunWith
import kotlin.Byte
import kotlin.Double
import kotlin.Float
import kotlin.random.Random

@RunWith(AndroidJUnit4::class)
class JsonTest {
    enum class Direction {
        NORTH,
    }

    @Test
    fun testAppInfoStatusFfiDisabled() {
        data class MyObj(
            var status: AppInfoStatusFfi,
        )

        val value = MyObj(status = AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.NeverStarted))
        val res = JSONObject(value.toJSONObject().toString())

        assertEquals(res.getJSONObject("status").getString("type"), "Disabled")
        assertEquals(
            res.getJSONObject("status").getJSONObject("reason").getString("type"),
            "NeverStarted",
        )
    }

    @Test
    fun testAppInfoStatusFfiPaused() {
        data class MyObj(
            var status: AppInfoStatusFfi,
        )

        val value = MyObj(status = AppInfoStatusFfi.Paused(PausedAppReasonFfi.Error("my error")))
        val res = JSONObject(value.toJSONObject().toString())

        assertEquals(res.getJSONObject("status").getString("type"), "Paused")
        assertEquals(res.getJSONObject("status").getJSONObject("reason").getString("type"), "Error")
        assertEquals(res.getJSONObject("status").getJSONObject("reason").getString("v1"), "my error")
    }

    @Test
    fun testCellInfoFfi() {
        data class MyObj(
            var info: CellInfoFfi,
        )

        val value =
            MyObj(
                info =
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
        val res = JSONObject(value.toJSONObject().toString())

        assertEquals(res.getJSONObject("info").getString("type"), "Provisioned")
        assertEquals(res.getJSONObject("info").getJSONObject("v1").getString("name"), "cell-1")
    }

    @Test
    fun testRoleSettingsFfi() {
        data class MyObj(
            var rolesettings: RoleSettingsFfi,
        )

        val value =
            MyObj(
                rolesettings =
                    RoleSettingsFfi.Provisioned(
                        membraneProof = ByteArray(60) { Random.nextInt(256).toByte() },
                        modifiers =
                            DnaModifiersOptFfi(
                                networkSeed = "1234",
                                properties = ByteArray(300) { Random.nextInt(256).toByte() },
                                originTime = 1000L,
                                quantumTime = DurationFfi(100UL, 100U),
                            ),
                    ),
            )

        val res = JSONObject(value.toJSONObject().toString())
        assertEquals(res.getJSONObject("rolesettings").getString("type"), "Provisioned")
        assertEquals(
            res.getJSONObject("rolesettings").getJSONObject("modifiers").getString("networkSeed"),
            "1234",
        )
    }

    @Test
    fun testPrimitives() {
        data class MyObj(
            var string: String,
            var int: Int,
            var uint: UInt,
            var long: Long,
            var ulong: ULong,
            var short: Short,
            var ushort: UShort,
            var double: Double,
            var float: Float,
            var boolean: Boolean,
            var byte: Byte,
            var ubyte: UByte,
            var myEnum: Direction,
            var empty: Int?,
            var byteArray: ByteArray,
        )
        val value =
            MyObj(
                string = "my string",
                int = 100,
                uint = 100U,
                long = 100L,
                ulong = 100UL,
                short = 100,
                ushort = 100U,
                double = 100.5,
                float = 100.5F,
                boolean = true,
                byte = 0x0,
                ubyte = 0x0.toUByte(),
                myEnum = Direction.NORTH,
                empty = null,
                byteArray = byteArrayOf(0x0, 0x1, 0x2),
            )
        val res = JSONObject(value.toJSONObject().toString())

        assertEquals(res.get("string"), "my string")
        assertEquals(res.get("int"), 100)
        assertEquals(res.get("uint"), 100)
        assertEquals(res.get("long"), 100)
        assertEquals(res.get("ulong"), 100)
        assertEquals(res.get("short"), 100)
        assertEquals(res.get("ushort"), 100)
        assertEquals(res.get("double"), 100.5)
        assertEquals(res.get("float"), 100.5)
        assertEquals(res.get("boolean"), true)
        assertEquals(res.get("byte"), 0)
        assertEquals(res.get("myEnum"), "NORTH")
        assertNull(res.optString("empty", null))
        assertEquals(res.get("byteArray").toString(), "[0,1,2]")
    }

    @Test
    fun testMap() {
        data class MyObj(
            var map: Map<String, String>,
        )
        val value = MyObj(map = mapOf("my key" to "my value"))
        val res = JSONObject(value.toJSONObject().toString())

        assertEquals(res.getJSONObject("map").getString("my key"), "my value")
    }

    @Test
    fun testCollection() {
        data class MyObj(
            var arr: List<String>,
        )
        val value = MyObj(arr = listOf("my string 1", "my string 2"))
        val res = JSONObject(value.toJSONObject().toString())

        assertEquals(res.getJSONArray("arr").getString(0), "my string 1")
        assertEquals(res.getJSONArray("arr").getString(1), "my string 2")
    }

    @Test
    fun testArray() {
        data class MyObj(
            var str: String,
        )
        val value = listOf(MyObj(str = "my string 1"), MyObj(str = "my string 2"))
        val res = JSONArray(value.toJSONArray().toString())

        assertEquals(res.getJSONObject(0).getString("str"), "my string 1")
        assertEquals(res.getJSONObject(1).getString("str"), "my string 2")
    }
}
