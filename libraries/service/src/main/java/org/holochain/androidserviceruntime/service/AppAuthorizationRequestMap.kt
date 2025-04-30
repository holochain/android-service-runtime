package org.holochain.androidserviceruntime.service

import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

data class AppAuthorizationRequest(
    // The android package requesting the authorization
    var clientPackageName: String,
    // The holochain app id that it wants to call
    var installedAppId: String,
)

// A map of ids to AppAuthorizationRequests
//
// The "approve" intent must include the id as extra data "requestId",
// to indicate which android package + holochain app pair should be approved.
@OptIn(ExperimentalUuidApi::class)
public class AppAuthorizationRequestMap {
    private var map: MutableMap<String, AppAuthorizationRequest> = mutableMapOf()

    fun put(request: AppAuthorizationRequest): String {
        var uuid = Uuid.random().toString()
        this.map.put(uuid, request)
        return uuid
    }

    fun pop(uuid: String): AppAuthorizationRequest? = this.map.remove(uuid)
}
