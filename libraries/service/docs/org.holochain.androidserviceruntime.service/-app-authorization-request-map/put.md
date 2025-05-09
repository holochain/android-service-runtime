//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[AppAuthorizationRequestMap](index.md)/[put](put.md)

# put

[androidJvm]\
fun [put](put.md)(request: [AppAuthorizationRequest](../-app-authorization-request/index.md)): [PutResponse](../-put-response/index.md)

Inserts a request into the map if not already included.

If the request is already in the map, returns the existing entry's information. Otherwise, creates a new entry with a unique UUID and notification ID.

#### Return

PutResponse containing the UUID and notification ID

#### Parameters

androidJvm

| | |
|---|---|
| request | The authorization request to add |