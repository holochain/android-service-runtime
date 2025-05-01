package org.holochain.androidserviceruntime.plugin.client

import android.app.Activity
import android.content.Intent
import android.util.Log
import android.widget.Button
import androidx.cardview.widget.CardView

class DisconnectedNotice(
    private val activity: Activity,
    private val servicePackage: String,
): OverlayNotice(
    activity,
    R.layout.disconnected_notice,
    "DisconnectedNotice"
) {
    override fun setupNoticeCardView(noticeView: CardView) {
        // Start the app that contains the HolochainService
        noticeView.findViewById<Button>(R.id.openSettingsAction).setOnClickListener {
            try {
                val launchIntent = this.activity.packageManager.getLaunchIntentForPackage(servicePackage)
                if (launchIntent != null) {
                    this.activity.startActivity(launchIntent)
                } else {
                    Log.e(logTag, "Could not find launch intent for package " + servicePackage)
                }
            } catch (e: Exception) {
                Log.e(logTag, "Failed to launch package " + servicePackage, e)
            }
        }

        // Restart this app
        noticeView.findViewById<Button>(R.id.restartAction).setOnClickListener {
            super.restartApp()
        }
    }
}
