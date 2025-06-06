package org.holochain.androidserviceruntime.plugin.client

import app.tauri.annotation.InvokeArg
import org.holochain.androidserviceruntime.client.CellIdFfi
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfi
import org.holochain.androidserviceruntime.client.RoleSettingsFfi
import org.holochain.androidserviceruntime.client.ZomeCallParamsFfi

@InvokeArg
class SetupAppConfigInvokeArg {
    lateinit var appId: String
    lateinit var happBundleBytes: ByteArray
    lateinit var networkSeed: String
    lateinit var rolesSettings: Map<String, RoleSettingsFfi>
    var enableAfterInstall: Boolean = true
}

fun SetupAppConfigInvokeArg.toInstallAppPayloadFfi(): InstallAppPayloadFfi =
    InstallAppPayloadFfi(
        source = this.happBundleBytes,
        installedAppId = this.appId,
        networkSeed = this.networkSeed,
        rolesSettings = this.rolesSettings,
    )

@InvokeArg
class AppIdInvokeArg {
    lateinit var installedAppId: String
}

@InvokeArg
class ZomeCallParamsFfiInvokeArg {
    lateinit var provenance: ByteArray
    lateinit var cellIdDnaHash: ByteArray
    lateinit var cellIdAgentPubKey: ByteArray
    lateinit var zomeName: String
    lateinit var fnName: String
    var capSecret: ByteArray? = null
    lateinit var payload: ByteArray
    lateinit var nonce: ByteArray
    var expiresAt: Long = 0L
}

fun ZomeCallParamsFfiInvokeArg.toFfi(): ZomeCallParamsFfi =
    ZomeCallParamsFfi(
        this.provenance,
        CellIdFfi(
            dnaHash = this.cellIdDnaHash,
            agentPubKey = this.cellIdAgentPubKey,
        ),
        this.zomeName,
        this.fnName,
        this.capSecret,
        this.payload,
        this.nonce,
        this.expiresAt,
    )
