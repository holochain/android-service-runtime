package org.holochain.androidserviceruntime.service

import java.util.concurrent.atomic.AtomicInteger

public class UniqueNotificationId {
    private var counter = AtomicInteger()

    public fun get(): Int = counter.getAndIncrement()
}
