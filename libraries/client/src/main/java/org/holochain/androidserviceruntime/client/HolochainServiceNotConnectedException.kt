package org.holochain.androidserviceruntime.client

/**
 * Exception thrown when attempting to use a Holochain service client without an active connection.
 *
 * @param message Error message (defaults to "Holochain service is not connected")
 * @param cause The cause of this exception (if any)
 */
class HolochainServiceNotConnectedException(
    message: String = "Holochain service is not connected",
    cause: Throwable? = null,
) : Exception(message, cause)
