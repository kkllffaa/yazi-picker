{
  description = "A development flake for a Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane, # Make sure crane is passed to outputs
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;
        crateInfo = craneLib.crateNameFromCargoToml {
          cargoToml = ./Cargo.toml;
        };
        commonArgs = {
          src = craneLib.cleanCargoSource ./.;
          # strictDeps = true;

          nativeBuildInputs = [
            pkgs.pkg-config
          ];

          # 2. The library itself
          buildInputs = [
            pkgs.dbus
          ];
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        my-crate = craneLib.buildPackage (
          commonArgs
          // crateInfo
          // {
            inherit cargoArtifacts;
          }
        );
      in
      {
        # Define the default package for nix build
        packages.default = my-crate;
        devShells.default = craneLib.devShell {
          checks = {
            inherit my-crate;
          };
          packages = with pkgs; [
            rust-analyzer
          ];
        };
        devShells.default_old = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rust-bin.stable.latest.default
            cargo
            rustc

            # Add crane's tools to the dev shell for convenience
            craneLib.cargoClippy
            craneLib.cargoFmt
            # craneLib.rust-analyzer # Uncomment if you want rust-analyzer
          ];

          shellHook = ''
            echo "Entering Rust development shell with crane..."
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "You can use 'crane build' to build your project (caches dependencies)."
            echo "Use 'cargo build' for a standard build within the environment."
          '';
        };
      }
    );
}
