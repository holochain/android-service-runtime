{
  description = "Flake for Android + Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix?ref=main-0.4";
    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";
    rust-overlay.follows = "holonix/rust-overlay";
  };
  outputs = inputs@{ flake-parts, nixpkgs, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { inputs', config, pkgs, system, lib, stdenv, ... }: {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default =
          let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
              inherit system overlays;
              config.allowUnfree = true;
            };

            rust = (pkgs.rust-bin.stable.latest.minimal.override
              {
                extensions = [ "clippy" "rustfmt" "rust-src" ];
                targets = [
                  "wasm32-unknown-unknown"
                  "aarch64-linux-android"
                  "x86_64-linux-android"
                  "i686-linux-android"
                  "armv7-linux-androideabi"
                ];
              });

              androidEnv = pkgs.androidenv.override { licenseAccepted = true; };
              androidComposition = androidEnv.composeAndroidPackages {
                includeNDK = true;
                includeEmulator = true;
                platformToolsVersion = "35.0.2";
                buildToolsVersions = [ "34.0.0" ];
                platformVersions = [ "34"];
                cmakeVersions = [ "3.10.2" ];
                extraLicenses = [
                  "android-googletv-license"
                  "android-sdk-arm-dbt-license"
                  "android-sdk-license"
                  "android-sdk-preview-license"
                  "google-gdk-license"
                  "intel-android-extra-license"
                  "intel-android-sysimage-license"
                  "mips-android-sysimage-license"
                ];
              };
          in
          pkgs.mkShell {
            packages = [
              rust
            ] ++ (with inputs'.holonix.packages; [
              holochain
              lair-keystore
              hc-launch
              hc-scaffold
              hn-introspect
            ]) ++ (with pkgs; [
              nodejs_20 # For UI development
              pnpm
              binaryen # For WASM optimisation
              
              # Repo Utils
              gnumake
              cargo-ndk

              # Android development
              androidComposition.androidsdk
              androidComposition.ndk-bundle
              gradle
              jdk17
            ]);

            # Tauri
            buildInputs = lib.optionals pkgs.stdenv.isLinux (with pkgs; [
              glibc
              libsoup
              cairo
              gtk3
              webkitgtk
            ]) ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
              CoreServices
              Security 
            ]);
            nativeBuildInputs = (with pkgs; [
              openssl.dev
              libsoup
              cairo
              atk
              gst_all_1.gstreamer
              gst_all_1.gst-plugins-base
              gst_all_1.gst-plugins-good
              gst_all_1.gst-plugins-bad
              librsvg
              webkitgtk_4_1
              pkg-config
            ]);

            NIX_LD = "${pkgs.stdenv.cc.libc}/lib/ld-linux-x86-64.so.2";
            ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
            NDK_HOME = "${androidComposition.androidsdk}/libexec/android-sdk/ndk/${builtins.head (pkgs.lib.lists.reverseList (builtins.split "-" "${androidComposition.ndk-bundle}"))}";
            ANDROID_SDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk";
            ANDROID_NDK_ROOT = "${androidComposition.androidsdk}/libexec/android-sdk/ndk-bundle";
            JAVA_HOME = pkgs.jdk17.home;

            shellHook = ''
              export PATH=$ANDROID_HOME/platform-tools:$ANDROID_HOME/build-tools:$ANDROID_HOME/tools:$PATH

              export PS1='\[\033[1;34m\][holonix-android:\w]\$\[\033[0m\] '
            '';
          };
      };
    };
}
