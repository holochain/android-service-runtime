package org.holochain.androidserviceruntime.service

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log
import org.holochain.androidserviceruntime.client.HolochainServiceIntentActions

/**
 * BroadcastReceiver that starts the HolochainService when the device has finished booting.
 */
class BootCompletedReceiver : BroadcastReceiver() {
    private val logTag = "HolochainService BootCompletedReceiver"

    /**
     * Called when the device has completed booting.
     *
     * This method checks for the ACTION_BOOT_COMPLETED intent and starts the HolochainService
     * with the ACTION_BOOT_COMPLETED action. The service will then decide whether to start
     * Holochain based on the user's autostart configuration.
     *
     * @param context The Context in which the receiver is running
     * @param intent The Intent being received
     */
    override fun onReceive(
        context: Context,
        intent: Intent,
    ) {
        if (intent.action != Intent.ACTION_BOOT_COMPLETED) {
            return
        }

        Log.d(logTag, "onReceive")

        val intent =
            Intent(context, HolochainService::class.java).apply {
                action = HolochainServiceIntentActions.ACTION_BOOT_COMPLETED
            }
        context.startForegroundService(intent)
    }
}
