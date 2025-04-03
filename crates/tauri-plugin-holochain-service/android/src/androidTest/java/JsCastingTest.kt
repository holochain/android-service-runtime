package com.plugin.holochain_service

import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.holochain.androidserviceruntime.holochain_service_client.AppInfoFfi
import org.holochain.androidserviceruntime.holochain_service_client.AppInfoFfiParcel
import org.holochain.androidserviceruntime.holochain_service_client.AppInfoStatusFfi
import org.holochain.androidserviceruntime.holochain_service_client.CellIdFfi
import org.holochain.androidserviceruntime.holochain_service_client.CellInfoFfi
import org.holochain.androidserviceruntime.holochain_service_client.DisabledAppReasonFfi
import org.holochain.androidserviceruntime.holochain_service_client.DnaModifiersFfi
import org.holochain.androidserviceruntime.holochain_service_client.DurationFfi
import org.holochain.androidserviceruntime.holochain_service_client.ProvisionedCellFfi

import org.junit.Test
import org.junit.runner.RunWith

import org.junit.Assert.*
import kotlin.random.Random

/**
 * Instrumented test, which will execute on an Android device.
 *
 * See [testing documentation](http://d.android.com/tools/testing).
 */
@RunWith(AndroidJUnit4::class)
class JSCastingTest {
    data class MyObj(
        var status: AppInfoStatusFfi
    )

    @Test
    fun testSealedClassHasType() {
        val value = MyObj(status = AppInfoStatusFfi.Disabled(DisabledAppReasonFfi.NeverStarted))
        val res = value.toJSObject()

        assertEquals(res.getJSObject("status")?.getString("type"), "Disabled")
    }
}
