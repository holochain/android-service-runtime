package org.holochain.androidserviceruntime.plugin.service

import app.tauri.annotation.InvokeArg
import org.holochain.androidserviceruntime.client.CellIdFfi
import org.holochain.androidserviceruntime.client.InstallAppPayloadFfi
import org.holochain.androidserviceruntime.client.RoleSettingsFfi
import org.holochain.androidserviceruntime.client.ZomeCallParamsFfi

@InvokeArg
class InstallAppPayloadFfiInvokeArg {
    lateinit var source: ByteArray
    lateinit var installedAppId: String
    lateinit var networkSeed: String
    lateinit var roleSettings: Map<String, RoleSettingsFfi>
}

fun InstallAppPayloadFfiInvokeArg.toFfi(): InstallAppPayloadFfi =
    InstallAppPayloadFfi(
        this.source,
        this.installedAppId,
        this.networkSeed,
        this.roleSettings,
    )

@InvokeArg
class AppIdInvokeArg {
    lateinit var installedAppId: String
}

@InvokeArg
class ZomeCallParamsFfiInvokeArg {
    lateinit var provenance: ByteArray
    lateinit var cellId: CellIdFfi
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
        this.cellId,
        this.zomeName,
        this.fnName,
        this.capSecret,
        this.payload,
        this.nonce,
        this.expiresAt,
    )
