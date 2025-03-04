# Unreleased

- Replace 3rd party `holochain_runtime` with `holochain-conductor-runtime`, modify FFI-bindings wrapper crate to use it. The runtime now makes calls via the `ConductorHandle` directly, without going through an `AdminWebsocket` to prevent unauthenticated access.
- Rename the FFI-bindings wrapper crate to `holochain-conductor-runtime-ffi` and generated kotlin bindings have been renamed accordingly.
- Removed the call `get_admin_port`, as an `AdminWebsocket` is no longer exposed.