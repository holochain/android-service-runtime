package org.holochain.androidserviceruntime.client

/**
 * Exception thrown when a client is not authorized to use the Admin API.
 *
 * @param message Error message (defaults to "AdminBinderUnauthorized")
 */
class AdminBinderUnauthorizedException(
    message: String = "AdminBinderUnauthorized",
) : Exception(message, null)

/**
 * Exception thrown when a client is not authorized to use the App API for a specific app.
 *
 * @param message Error message (defaults to "AppBinderUnauthorized")
 */
class AppBinderUnauthorizedException(
    message: String = "AppBinderUnauthorized",
) : Exception(message, null)
