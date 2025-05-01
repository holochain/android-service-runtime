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

abstract class OverlayNotice(
    private val activity: Activity,
    private val noticeCardViewLayoutRes: Int,
    internal val logTag: String = "OverlayNotice"
) {
    private var showOnLoad: Boolean = false
    private var noticeView: CardView? = null
    private var blurView: FrameLayout? = null

    /**
     * Call when plugin is loaded
     */
    fun load() {
        if (this.showOnLoad) {
            this.showOnLoad = false
            this.show()
        }
    }

    /**
     * Enable show on load
     *
     * Call when you want to trigger loading, but the webview has not been initialized yet.
     */
    fun enableShowOnLoad() {
        this.showOnLoad = true
    }

    /**
     * Remove notice
     */
    fun hide() {
        Log.d(logTag, "hide")
        showOnLoad = false
        activity.runOnUiThread {
            try {
                // Clear notification reference
                noticeView = null

                // Remove overlay
                blurView?.let {
                    val rootView = activity.findViewById<ViewGroup>(android.R.id.content)
                    rootView.removeView(it)
                    blurView = null
                    Log.d(logTag, "Blur overlay and notification removed")
                }
            } catch (e: Exception) {
                Log.e(logTag, "Failed to remove blur overlay", e)
            }
        }
    }

    /**
     * Display notice
     */
    fun show() {
        Log.d(logTag, "show")

        // Wait until the webView is available before displaying the notice,
        // otherwise the notice will not display.
        if (this.showOnLoad) {
            return
        }

        try {
            activity.runOnUiThread {
                // Create Blur Background View
                blurView = FrameLayout(activity)
                blurView!!.layoutParams =
                    FrameLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT,
                        ViewGroup.LayoutParams.MATCH_PARENT,
                    )
                blurView!!.background = ColorDrawable(Color.parseColor("#80000000"))
                blurView!!.isClickable = true
                blurView!!.isFocusable = true
                blurView!!.setOnClickListener { }

                // Create Notice View
                val noticeView = LayoutInflater.from(activity).inflate(this.noticeCardViewLayoutRes, null) as CardView
                val layoutParams =
                    FrameLayout.LayoutParams(
                        ViewGroup.LayoutParams.MATCH_PARENT,
                        ViewGroup.LayoutParams.WRAP_CONTENT,
                    )
                layoutParams.gravity = Gravity.CENTER
                layoutParams.leftMargin = 48
                layoutParams.rightMargin = 48
                blurView!!.addView(noticeView, layoutParams)

                // Setup CardView actions
                this.setupNoticeCardView(noticeView);

                // Render views
                activity.findViewById<ViewGroup>(android.R.id.content).addView(blurView)
            }
        } catch (e: Exception) {
            Log.e(logTag, "Failed to show notice", e)
        }
    }

    // Restart the android app this notice is displayed within
    internal fun restartApp() {
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

    // This can be overriden by subclasses, to provide custom handling of CardView actions
    open fun setupNoticeCardView(noticeView: CardView) {}
}
