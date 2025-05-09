package org.holochain.androidserviceruntime.service

import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

/**
 * Represents a request from an Android app to access a Holochain app.
 */
data class AppAuthorizationRequest(
    /**
     * The Android package name requesting the authorization
     */
    var clientPackageName: String,
    /**
     * The Holochain app ID that the client wants to access
     */
    var installedAppId: String,
)

/**
 * Associates an app authorization request with a notification ID.
 *
 * @property request The authorization request
 * @property notificationId The ID of the notification shown to the user
 */
data class AppAuthorizationRequestNotification(
    var request: AppAuthorizationRequest,
    var notificationId: Int,
)

/**
 * Response from adding an authorization request to the map.
 *
 * @property uuid The unique identifier assigned to the request
 * @property notificationId The notification ID assigned to the request
 */
data class PutResponse(
    var uuid: String,
    var notificationId: Int,
)

/**
 * A map of IDs to AppAuthorizationRequests and their associated notification IDs.
 *
 * The "approve" intent must include the ID as extra data "requestId",
 * to indicate which Android package + Holochain app pair should be approved.
 *
 * @param initialNotificationId The starting notification ID to use
 */
@OptIn(ExperimentalUuidApi::class)
public class AppAuthorizationRequestMap(
    initialNotificationId: Int,
) {
    private var map: MutableMap<String, AppAuthorizationRequestNotification> = mutableMapOf()
    private val notificationIdGenerator = NotificationIdGenerator(initialNotificationId)

    /**
     * Checks if the request is already in the map.
     *
     * @param request The authorization request to look for
     * @return PutResponse containing the UUID and notification ID if found, null otherwise
     */
    private fun mapContainsRequest(request: AppAuthorizationRequest): PutResponse? {
        val entry = this.map.entries.find { it.value.request == request }
        if (entry == null) {
            return null
        }

        return PutResponse(entry.key, entry.value.notificationId)
    }

    /**
     * Inserts a request into the map if not already included.
     *
     * If the request is already in the map, returns the existing entry's information.
     * Otherwise, creates a new entry with a unique UUID and notification ID.
     *
     * @param request The authorization request to add
     * @return PutResponse containing the UUID and notification ID
     */
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

    /**
     * Gets and removes a request from the map by its UUID.
     *
     * @param uuid The unique identifier of the request to retrieve and remove
     * @return The AppAuthorizationRequestNotification if found, null otherwise
     */
    fun pop(uuid: String): AppAuthorizationRequestNotification? = this.map.remove(uuid)
}
