package org.holochain.androidserviceruntime.plugin.client

import android.app.Activity
import android.widget.Button
import android.widget.TextView
import androidx.cardview.widget.CardView

class UnauthorizedNotice(
    private val activity: Activity,
    private val servicePackage: String,
) : Notice(
        activity,
        R.layout.unauthorized_notice,
        "UnauthorizedNotice",
    ) {
    private var installedAppId: String? = ""

    override fun setupNoticeCardView(noticeView: CardView) {
        noticeView.findViewById<TextView>(R.id.installedAppId).setText(this.installedAppId)

        noticeView.findViewById<Button>(R.id.restartAction).setOnClickListener {
            super.restartApp()
        }
    }

    fun setInstalledAppId(value: String) {
        this.installedAppId = value
    }
}
