# CHANGELOG

## Unreleased

- Feat: Enable the feature flag `system_settings` to override AndroidManifest.xml such that app is included in the Android System Settings home page, instead of the App Grid like a typical user app.
- Feat: Add nix flake for easy dev environment setup
- Chore: Tauri plugins for managing and connecting to holochain service have been migrated into this repo.
- CI: Run static analysis and tests for linux targets for crates  `holochain-conductor-runtime` and `holochain-conductor-runtime-ffi`.