{
  description = "A development flake for a Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        # pkgs = nixpkgs.legacyPackages.${system};
        pkgs = import nixpkgs { inherit system; };

        myPackage = pkgs.callPackage ./yazi-portal-rust { };
      in
      {
        # Define the default package for nix build
        packages.default = myPackage;

        devShells.default = pkgs.mkShell {
          # inputsFrom = [ myPackage ];

          buildInputs = with pkgs; [
            cargo
            rustc

            # rust-rust-analyzer
            # rustfmt

            # Add crane's tools to the dev shell for convenience
            # craneLib.cargoClippy
            # craneLib.cargoFmt
            # craneLib.rust-analyzer # Uncomment if you want rust-analyzer
          ];
          # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
