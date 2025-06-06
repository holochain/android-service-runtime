{
  description = "Template for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix/main-0.5";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";

    tauri-plugin-holochain.url = "github:darksoil-studio/tauri-plugin-holochain/main-0.5";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = builtins.attrNames inputs.holonix.devShells;
      perSystem = { inputs', config, pkgs, system, ... }: {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ 
            inputs'.tauri-plugin-holochain.devShells.holochainTauriAndroidDev 
            inputs'.holonix.devShells.default
          ];
          
          packages = [
            pkgs.cargo-ndk
            pkgs.pnpm
          ];
        };
      };
    };
}