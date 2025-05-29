{
  description = "Flake for Android + Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix?ref=main-0.4";
    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";
    rust-overlay.follows = "holonix/rust-overlay";

    android-nixpkgs = {
      url = "github:tadfisher/android-nixpkgs";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ flake-parts, nixpkgs, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { inputs', config, pkgs, system, ... }: {
        formatter = pkgs.nixpkgs-fmt;

        devShells.default =
          let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
              inherit system overlays;
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

            android = inputs.android-nixpkgs.sdk.${system} (sdkPkgs: with sdkPkgs; [
              cmdline-tools-latest
              build-tools-34-0-0
              platform-tools
              platforms-android-34
              ndk-27-2-12479018
            ]);
          in
          pkgs.mkShell {
            packages = [
              rust
            ] ++ (with inputs'.holonix.packages; [
              # For the example-client-app
              holochain
              lair-keystore
              hc-launch
              hc-scaffold
              hn-introspect
            ]) ++ (with pkgs; [
              nodejs_20 # For UI development
              pnpm
              binaryen # For WASM optimisation

              # Project Utils
              gnumake

              # Android development
              android
              gradle
              jdk17
              cargo-ndk

              # Tauri
              curl
              xz
            ]);

            # Tauri
            buildInputs = (with pkgs; [
              glibc
              libsoup
              cairo
              atk
              webkitgtk_4_1
              openssl
              librsvg
            ]);
            nativeBuildInputs = (with pkgs; [
              pkg-config
            ]);

            ANDROID_HOME = "${android}/share/android-sdk";
            NDK_HOME = "${android}/share/android-sdk/ndk/27.2.12479018";
            JAVA_HOME = pkgs.jdk17.home;

            shellHook = ''
              export PS1='\[\033[1;34m\][holonix-android:\w]\$\[\033[0m\] '
            '';
          };
      };
    };
}
