{
  description = "A development flake for a Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
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

            rust-analyzer
            fenix.packages.${system}.latest.rustfmt
          ];
          # RUSTFMT = "${fenix.packages.${system}.latest.rustfmt}/bin/rustfmt";
          # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
