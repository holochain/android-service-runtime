package org.holochain.androidserviceruntime.plugin.client

import android.app.Activity
import android.content.Intent
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.util.Log
import android.view.Gravity
import android.view.LayoutInflater
import android.view.ViewGroup
import android.widget.Button
import android.widget.FrameLayout
import androidx.cardview.widget.CardView

class UnauthorizedNotice(
    private val activity: Activity,
    private val servicePackage: String,
): OverlayNotice(
    activity,
    R.layout.disconnected_notice,
    "UnauthorizedNotice"
) {
    override fun setupNoticeCardView(noticeView: CardView) {
        Log.d(this.logTag, "setupNoticeCardView")
    }

    private fun relaunch() {
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
