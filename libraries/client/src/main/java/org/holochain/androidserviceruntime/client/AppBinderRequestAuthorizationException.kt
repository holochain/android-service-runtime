package org.holochain.androidserviceruntime.client

class AppBinderRequestAuthorizationException(
    requestId: String,
    message: String = "Unauthorized, requesting authorization",
) : Exception(message, null)
