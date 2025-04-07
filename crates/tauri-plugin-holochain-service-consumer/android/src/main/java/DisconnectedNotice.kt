package com.plugin.holochain_service_consumer

import android.app.Activity
import android.content.Intent
import android.graphics.Color
import android.graphics.drawable.ColorDrawable
import android.view.Gravity
import android.view.LayoutInflater
import android.view.ViewGroup
import android.util.Log
import android.widget.Button
import android.widget.FrameLayout
import android.widget.TextView
import androidx.cardview.widget.CardView

class DisconnectedNotice(private val activity: Activity, private val servicePackage: String) {
    private var showOnLoad: Boolean = false
    private var noticeView: CardView? = null
    private var blurView: FrameLayout? = null
    private val TAG = "DisconnectedNotice"

    /**
     * Call when plugin is loaded
     */
    fun load() {
        if(this.showOnLoad) {
            this.showOnLoad = false
            this.show()
        }
    }

    fun enableShowOnLoad() {
        this.showOnLoad = true
    }

    /**
     * Removes the notice and blur background
     */
    fun hide() {
        Log.d(TAG, "hide")
        showOnLoad = false;
        activity.runOnUiThread {
            try {
                // Clear notification reference
                noticeView = null
                
                // Remove overlay
                blurView?.let {
                    val rootView = activity.findViewById<ViewGroup>(android.R.id.content)
                    rootView.removeView(it)
                    blurView = null
                    Log.d(TAG, "Blur overlay and notification removed")
                }
            } catch (e: Exception) {
                Log.e(TAG, "Failed to remove blur overlay", e)
            }
        }
    }

    /**
     * Shows a centered notification that Holochain Service is not running
     */
    fun show() {
        Log.d(TAG, "show")

        // Wait until the webView is available before displaying the notice
        // Otherwise we are not able to reload the webview when reloadAction is pressed
        if(this.showOnLoad) {
            return;
        }

        try {
            // Show blur overlay and custom notification on UI thread
            activity.runOnUiThread {
                // Create Blur Background View
                blurView = FrameLayout(activity)
                blurView!!.layoutParams = FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.MATCH_PARENT
                )
                blurView!!.background = ColorDrawable(Color.parseColor("#80000000"))
                blurView!!.isClickable = true
                blurView!!.isFocusable = true
                blurView!!.setOnClickListener { }

                // Create Notice View
                val noticeView = LayoutInflater.from(activity).inflate(R.layout.disconnected_notice, null)  as CardView
                val layoutParams = FrameLayout.LayoutParams(
                    ViewGroup.LayoutParams.MATCH_PARENT,
                    ViewGroup.LayoutParams.WRAP_CONTENT
                )
                layoutParams.gravity = Gravity.CENTER
                layoutParams.leftMargin = 48
                layoutParams.rightMargin = 48
                blurView!!.addView(noticeView, layoutParams)

                // Button Callbacks
                noticeView.findViewById<Button>(R.id.openSettingsAction).setOnClickListener {
                    // Start android-service-runtime package
                    try {
                        val launchIntent = this.activity.packageManager.getLaunchIntentForPackage(servicePackage)
                        if (launchIntent != null) {
                            this.activity.startActivity(launchIntent)
                        } else {
                            Log.e(TAG, "Could not find launch intent for package " + servicePackage)
                        }
                    } catch (e: Exception) {
                        Log.e(TAG, "Failed to launch package " + servicePackage, e)
                    }
                }
                noticeView.findViewById<Button>(R.id.reloadAction).setOnClickListener {
                    // Restart this package
                    try {
                        val packageManager = this.activity.packageManager
                        val intent = packageManager.getLaunchIntentForPackage(this.activity.packageName);
                        val componentName = intent!!.getComponent();
                        val mainIntent = Intent.makeRestartActivityTask(componentName);
                        
                        mainIntent.setPackage(this.activity.packageName);
                        this.activity.startActivity(mainIntent);
                        Runtime.getRuntime().exit(0);
                    } catch (e: Exception) {
                        Log.e(TAG, "Failed to restart app", e)
                    }
                }

                // Attach to root view
                activity.findViewById<ViewGroup>(android.R.id.content).addView(blurView)
            }
        } catch (e: Exception) {
            // Log failure but don't crash
            Log.e(TAG, "Failed to show notification", e)
        }
    }
}