package org.holochain.androidserviceruntime.plugin.client

import android.app.Activity
import android.content.ComponentName
import android.util.Log
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import org.holochain.androidserviceruntime.client.HolochainServiceAppClient
import org.holochain.androidserviceruntime.client.HolochainServiceNotConnectedException
import org.holochain.androidserviceruntime.client.AppBinderUnauthorizedException
import org.holochain.androidserviceruntime.client.toJSONObjectString

@TauriPlugin
class HolochainServiceClientPlugin(
    private val activity: Activity,
) : Plugin(activity) {
    private val supervisorJob = SupervisorJob()
    private val serviceScope = CoroutineScope(supervisorJob)
    private val servicePackage = "org.holochain.androidserviceruntime.app"
    private val serviceClient =
        HolochainServiceAppClient(
            this.activity,
            ComponentName(servicePackage, "org.holochain.androidserviceruntime.service.HolochainService"),
        )
    private val disconnectedNotice = DisconnectedNotice(activity, servicePackage)
    private val unauthorizedNotice = UnauthorizedNotice(activity, servicePackage)
    private val logTag = "HolochainServiceClientPlugin"
    private var webView: WebView? = null

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        Log.d(logTag, "load")
        super.load(webView)
        this.webView = webView

        disconnectedNotice.load()
        unauthorizedNotice.load()
    }

    /**
     * Connect to service, wait for connection to be ready, then setup an app
     */
    @Command
    fun connectSetupApp(invoke: Invoke) {
        Log.d(logTag, "connectSetupApp")
        val args = invoke.parseArgs(SetupAppConfigInvokeArg::class.java)
        unauthorizedNotice.setInstalledAppId(args.appId)

        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.connectSetupApp(args.toInstallAppPayloadFfi(), args.enableAfterInstall)
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                handleCommandException(e, invoke)
            }
        }
    }

    /**
     * Enable an app
     */
    @Command
    fun enableApp(invoke: Invoke) {
        Log.d(logTag, "enableApp")
        val args = invoke.parseArgs(AppIdInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.enableApp(args.installedAppId)
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                handleCommandException(e, invoke)
            }
        }
    }

    /**
     * Sign Zome Call
     */
    @Command
    fun signZomeCall(invoke: Invoke) {
        Log.d(logTag, "signZomeCall")
        val args = invoke.parseArgs(ZomeCallUnsignedFfiInvokeArg::class.java)
        serviceScope.launch(Dispatchers.IO) {
            try {
                val res = serviceClient.signZomeCall(args.toFfi())
                invoke.resolve(JSObject(res.toJSONObjectString()))
            } catch (e: Exception) {
                handleCommandException(e, invoke)
            }
        }
    }

    /**
     * Display the service notice if the exception is HolochainServiceNotConnectedException
     */
    private fun handleCommandException(
        e: Exception,
        invoke: Invoke,
    ) {
        Log.d(logTag, "handleCommandException")
        if (e is HolochainServiceNotConnectedException) {
            Log.d(logTag, "handleCommandException HolochainServiceNotConnectedException")
            if (this.webView == null) {
                disconnectedNotice.enableShowOnLoad()
            } else {
                disconnectedNotice.show()
            }
            invoke.reject(e.toString(), "HolochainServiceNotConnected")
        } else if (e is AppBinderUnauthorizedException) {
            Log.d(logTag, "handleCommandException AppBinderUnauthorizedException")
            if (this.webView == null) {
                unauthorizedNotice.enableShowOnLoad()
            } else {
                unauthorizedNotice.show()
            }
            invoke.reject(e.toString(), "HolochainServiceNotConnected")
        } else {
            invoke.reject(e.toString())
        }
    }
}
