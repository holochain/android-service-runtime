package org.holochain.androidserviceruntime.client

class AppBinderUnauthorizedException(
    message: String = "Unauthorized",
    cause: Throwable? = null,
) : Exception(message, cause)
