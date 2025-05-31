//[client](../../../index.md)/[org.holochain.androidserviceruntime.client](../index.md)/[HolochainServiceAdminClient](index.md)/[signZomeCall](sign-zome-call.md)

# signZomeCall

[androidJvm]\
suspend fun [signZomeCall](sign-zome-call.md)(args: [ZomeCallParamsFfi](../-zome-call-params-ffi/index.md)): [ZomeCallParamsSignedFfi](../-zome-call-params-signed-ffi/index.md)

Signs a zome call with the agent's private key.

This is required for making authenticated calls to Holochain zome functions.

#### Return

The signed zome call ready to be executed

#### Parameters

androidJvm

| | |
|---|---|
| args | The unsigned zome call to sign |

#### Throws

| | |
|---|---|
| [HolochainServiceNotConnectedException](../-holochain-service-not-connected-exception/index.md) | if not connected to the service |