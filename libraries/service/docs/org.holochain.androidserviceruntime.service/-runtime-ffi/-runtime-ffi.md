//[service](../../../index.md)/[org.holochain.androidserviceruntime.service](../index.md)/[RuntimeFfi](index.md)/[RuntimeFfi](-runtime-ffi.md)

# RuntimeFfi

[androidJvm]\
constructor(pointer: Pointer)

[androidJvm]\
constructor(noPointer: NoPointer)

This constructor can be used to instantiate a fake object. Only used for tests. Any attempt to actually use an object constructed this way will fail as there is no connected Rust object.