package org.holochain.androidserviceruntime.service

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.util.Log

/**
 * BroadcastReceiver that starts the HolochainService when the device has finished booting.
 */
class BootCompletedReceiver : BroadcastReceiver() {
    private val logTag = "HolochainService BootCompletedReceiver"

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
                action = HolochainService.ACTION_BOOT_COMPLETED
            }
        context.startForegroundService(intent)
    }
}
