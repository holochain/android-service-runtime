package org.holochain.androidserviceruntime.service

import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

// A request from an android package to use a holochain app
data class AppAuthorizationRequest(
    // The android package requesting the authorization
    var clientPackageName: String,
    // The holochain app id that it wants to call
    var installedAppId: String,
)

data class AppAuthorizationRequestNotification(
    var request: AppAuthorizationRequest,
    var notificationId: Int,
)

data class PutResponse(
    var uuid: String,
    var notificationId: Int,
)

// A map of ids to AppAuthorizationRequests + notificationId
//
// The "approve" intent must include the id as extra data "requestId",
// to indicate which android package + holochain app pair should be approved.
@OptIn(ExperimentalUuidApi::class)
public class AppAuthorizationRequestMap(
    initialNotificationId: Int,
) {
    private var map: MutableMap<String, AppAuthorizationRequestNotification> = mutableMapOf()
    private val notificationIdGenerator = NotificationIdGenerator(initialNotificationId)

    // If request is already found in map, returns the key and notificationId,
    // Otherwise return null
    private fun mapContainsRequest(request: AppAuthorizationRequest): PutResponse? {
        val entry = this.map.entries.find { it.value.request == request }
        if (entry == null) {
            return null
        }

        return PutResponse(entry.key, entry.value.notificationId)
    }

    // Insert request into map, if not already included
    fun put(request: AppAuthorizationRequest): PutResponse {
        var putResponse = this.mapContainsRequest(request)

        if (putResponse == null) {
            var uuid = Uuid.random().toString()
            var notificationId = notificationIdGenerator.next()
            putResponse = PutResponse(uuid, notificationId)

            this.map.put(uuid, AppAuthorizationRequestNotification(request, notificationId))
        }

        return putResponse
    }

    // Get and remove a request from the map, by its uuid
    fun pop(uuid: String): AppAuthorizationRequestNotification? = this.map.remove(uuid)
}
