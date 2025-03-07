package com.plugin.holochain_service_consumer

import app.tauri.annotation.InvokeArg
import com.holochain_apps.holochain_service_types.CellIdFfi
import com.holochain_apps.holochain_service_types.InstallAppPayloadFfi
import com.holochain_apps.holochain_service_types.RoleSettingsFfi
import com.holochain_apps.holochain_service_types.ZomeCallUnsignedFfi

@InvokeArg
class InstallAppPayloadFfiInvokeArg {
    lateinit var source: ByteArray
    lateinit var installedAppId: String
    lateinit var networkSeed: String
    lateinit var roleSettings: Map<String, RoleSettingsFfi>
}

fun InstallAppPayloadFfiInvokeArg.toFfi(): InstallAppPayloadFfi {
    return InstallAppPayloadFfi(
        this.source,
        this.installedAppId,
        this.networkSeed,
        this.roleSettings,
    )
}

@InvokeArg
class AppIdInvokeArg {
    lateinit var installedAppId: String
}

@InvokeArg
class ZomeCallUnsignedFfiInvokeArg {
    lateinit var provenance: ByteArray
    lateinit var cellId: CellIdFfi
    lateinit var zomeName: String
    lateinit var fnName: String
    var capSecret: ByteArray? = null
    lateinit var payload: ByteArray
    lateinit var nonce: ByteArray
    var expiresAt: Long = 0L
}

fun ZomeCallUnsignedFfiInvokeArg.toFfi(): ZomeCallUnsignedFfi {
    return ZomeCallUnsignedFfi(
        this.provenance,
        this.cellId,
        this.zomeName,
        this.fnName,
        this.capSecret,
        this.payload,
        this.nonce,
        this.expiresAt
    )
}