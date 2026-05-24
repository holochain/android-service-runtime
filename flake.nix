{
  description = "Holochain Android Service Runtime — development environment";

  # We depend on holonix only for the Holochain toolchain (holochain, hc,
  # hc-scaffold, lair-keystore, ...) and the rust-overlay it already pins.
  # Everything else (Rust toolchain, Android SDK/NDK, Tauri desktop libs) is
  # composed here from nixpkgs, so the repo has no dependency on any external
  # Tauri/Holochain dev-shell flake.
  inputs = {
    holonix.url = "github:holochain/holonix/main-0.6";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";
    rust-overlay.follows = "holonix/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { system, inputs', ... }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
            config = {
              allowUnfree = true; # Android SDK/NDK are unfree
              android_sdk.accept_license = true;
            };
          };

          # Rust toolchain. Channel, components, and cross-compilation targets all
          # come from ./rust-toolchain.toml so nix and rustup users stay in sync.
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          # Android SDK + NDK, matching the gradle config (compileSdk/buildTools 34,
          # minSdk 27). The NDK is what `cargo ndk` uses to cross-compile the Rust
          # crates into the jniLibs consumed by the Android libraries/plugins.
          ndkVersion = "26.1.10909125";
          androidComposition = pkgs.androidenv.composeAndroidPackages {
            platformVersions = [ "34" ];
            buildToolsVersions = [ "34.0.0" ];
            includeNDK = true;
            ndkVersions = [ ndkVersion ];
            cmakeVersions = [ "3.22.1" ];
            includeEmulator = false; # emulator is provided by CI / installed on demand
            includeSystemImages = false;
          };
          androidSdk = androidComposition.androidsdk;
          androidHome = "${androidSdk}/libexec/android-sdk";
          ndkHome = "${androidHome}/ndk/${ndkVersion}";

          # System libraries to build/run a Tauri v2 desktop app on Linux. Needed
          # for the desktop-first unified plugin work; harmless for Android builds.
          tauriDeps = with pkgs; [
            gtk3
            webkitgtk_4_1
            libsoup_3
            librsvg
            openssl
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            # Tools placed on PATH.
            packages = (with inputs'.holonix.packages; [
              holochain
              hc
              hc-scaffold
              lair-keystore
              bootstrap-srv
            ]) ++ [
              rust
              androidSdk
            ] ++ (with pkgs; [
              cargo-ndk # build Rust -> Android jniLibs
              nodejs_20
              pnpm
              jdk17 # Gradle
              pkg-config
              binaryen # wasm-opt, for building hApp/zome wasm
            ]);

            # Libraries to compile/link against (exposed via pkg-config).
            buildInputs = tauriDeps;

            shellHook = ''
              export ANDROID_HOME="${androidHome}"
              export ANDROID_SDK_ROOT="${androidHome}"
              export ANDROID_NDK="${ndkHome}"
              export ANDROID_NDK_ROOT="${ndkHome}"
              export ANDROID_NDK_HOME="${ndkHome}"
              export NDK_HOME="${ndkHome}"

              # Visual cue that you are inside the dev shell.
              export PS1='\[\033[1;35m\][asr-dev:\w]\$\[\033[0m\] '
            '';
          };
        };
    };
}
