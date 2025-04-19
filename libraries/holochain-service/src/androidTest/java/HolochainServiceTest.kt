package com.plugin.holochain_service

import android.content.Context
import android.content.Intent
import androidx.test.core.app.ApplicationProvider
import androidx.test.rule.ServiceTestRule
import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceAdmin
import org.holochain.androidserviceruntime.holochain_service_client.IHolochainServiceApp
import org.junit.Assert.*
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import java.security.InvalidParameterException

class HolochainServiceTest {

    @get:Rule
    val serviceRule = ServiceTestRule()

    @Test
    fun bindIntentWithoutApiFails() {
        val intent = Intent(
            ApplicationProvider.getApplicationContext<Context>(),
            HolochainService::class.java
        )

        assertThrows(InvalidParameterException::class.java) {
            HolochainService().onBind(intent)
        }
    }

    @Test
    fun bindIntentWithApiAdmin() {
        val serviceIntent = Intent(
            ApplicationProvider.getApplicationContext<Context>(),
            HolochainService::class.java
        )
        serviceIntent.putExtra("api", "admin")

        val binder = serviceRule.bindService(serviceIntent)
        assert(binder is IHolochainServiceAdmin)
    }

    @Test
    fun bindIntentWithApiAppFails() {
        val intent = Intent(
            ApplicationProvider.getApplicationContext<Context>(),
            HolochainService::class.java
        )
        intent.putExtra("api", "app")

        assertThrows(InvalidParameterException::class.java) {
            HolochainService().onBind(intent)
        }
    }

    @Test
    fun bindIntentWithApiAppAndInstalledAppId() {
        val serviceIntent = Intent(
            ApplicationProvider.getApplicationContext<Context>(),
            HolochainService::class.java
        )
        serviceIntent.putExtra("api", "app")
        serviceIntent.putExtra("installedAppId", "my-app-1")

        val binder = serviceRule.bindService(serviceIntent)
        assert(binder is IHolochainServiceApp)
    }

    // TODO
    // - test we can call admin binder functions from same package
    // - test we cannot call admin binder functions from different package
    // - test we can call app binder functions from authorized package
    // - test we cannot call app binder functions from unauthorized package
}

