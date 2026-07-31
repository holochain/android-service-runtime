{
  description = "Holochain Android Service Runtime — development environment";

  # We depend on holonix only for the Holochain toolchain (holochain, hc,
  # hc-scaffold, lair-keystore, ...) and the rust-overlay it already pins.
  # Everything else (Rust toolchain, Android SDK/NDK, Tauri desktop libs) is
  # composed here from nixpkgs, so the repo has no dependency on any external
  # Tauri/Holochain dev-shell flake.
  inputs = {
    holonix.url = "github:holochain/holonix/main-0.7";

    # webkitgtk pin for the desktop dev shell. holonix's nixpkgs ships webkitgtk
    # 2.52.x, which aborts with "Could not create default EGL display:
    # EGL_BAD_PARAMETER" and renders a blank Tauri webview on non-NixOS GPUs during
    # `tauri dev`. This nixpkgs rev provides webkitgtk 2.42.5, which renders
    # correctly. Dev-shell only: production bundles link the system/CI webkit, not
    # this.
    webkitnixpkgs.url = "github:nixos/nixpkgs/ed4db9c6c75079ff3570a9e3eb6806c8f692dc26";

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

          # GTK/webkit stack from the pinned (older, working) nixpkgs — see the
          # `webkitnixpkgs` input comment. Sourced here rather than from `pkgs` so
          # `tauri dev` renders instead of aborting on EGL.
          webkitPkgs = inputs.webkitnixpkgs.legacyPackages.${system};

          # System libraries to build/run a Tauri v2 desktop app on Linux. Needed
          # for the desktop-first unified plugin work; harmless for Android builds.
          # The GTK/webkit libs come from webkitPkgs (2.42.5); openssl from pkgs.
          tauriDeps = (with webkitPkgs; [
            webkitgtk_4_1
            gtk3
            gdk-pixbuf
            glib
            glib-networking
            librsvg
            libsoup_3
            dbus
          ]) ++ (with pkgs; [ openssl ]);
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
              nodejs_22
              pnpm
              jdk17 # Gradle
              pkg-config
              binaryen # wasm-opt, for building hApp/zome wasm
            ]) ++ (with webkitPkgs; [
              shared-mime-info
              gsettings-desktop-schemas
            ]);

            # GTK app wrapper hook: wires GSETTINGS / GIO / GDK-pixbuf / XDG paths at
            # shell entry so the webview finds its resources.
            nativeBuildInputs = [ webkitPkgs.wrapGAppsHook ];

            # Libraries to compile/link against (exposed via pkg-config).
            buildInputs = tauriDeps;

            shellHook = ''
              export ANDROID_HOME="${androidHome}"
              export ANDROID_SDK_ROOT="${androidHome}"
              export ANDROID_NDK="${ndkHome}"
              export ANDROID_NDK_ROOT="${ndkHome}"
              export ANDROID_NDK_HOME="${ndkHome}"
              export NDK_HOME="${ndkHome}"

              # GTK/webkit runtime so `tauri dev` renders (see the webkitnixpkgs input).
              export GIO_MODULE_DIR=${webkitPkgs.glib-networking}/lib/gio/modules/
              export GIO_EXTRA_MODULES=${webkitPkgs.glib-networking}/lib/gio/modules
              # Force software compositing by default so the webview renders on finicky
              # GPUs/drivers. Set ENABLE_WEBKIT_COMPOSITING=1 before `nix develop` to
              # keep hardware-accelerated compositing instead.
              if [ -z "$ENABLE_WEBKIT_COMPOSITING" ]; then
                export WEBKIT_DISABLE_COMPOSITING_MODE=1
              fi
              export XDG_DATA_DIRS=${webkitPkgs.shared-mime-info}/share:${webkitPkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${webkitPkgs.gsettings-desktop-schemas.name}:${webkitPkgs.gtk3}/share/gsettings-schemas/${webkitPkgs.gtk3.name}:$XDG_DATA_DIRS

              # Visual cue that you are inside the dev shell.
              export PS1='\[\033[1;35m\][asr-dev:\w]\$\[\033[0m\] '
            '';
          };
        };
    };
}
