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
        // Setup Button Callbacks
        noticeView.findViewById<Button>(R.id.openSettingsAction).setOnClickListener {
            // Start android-service-runtime package
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
        noticeView.findViewById<Button>(R.id.denyAction).setOnClickListener {
            // Restart this package
            try {
                val packageManager = this.activity.packageManager
                val intent = packageManager.getLaunchIntentForPackage(this.activity.packageName)
                val componentName = intent!!.getComponent()
                val mainIntent = Intent.makeRestartActivityTask(componentName)

                mainIntent.setPackage(this.activity.packageName)
                this.activity.startActivity(mainIntent)
                Runtime.getRuntime().exit(0)
            } catch (e: Exception) {
                Log.e(logTag, "Failed to restart app", e)
            }
        }
    }
}
