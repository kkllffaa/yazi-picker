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
        packages.portal = myPackage;

        packages.plugin = pkgs.stdenvNoCC.mkDerivation rec {
          pname = "yazi-picker-plugin";
          name = pname;
          src = ./smart-picker.yazi;
          installPhase = ''
            runHook preInstall
            cp -r ${src} $out
            runHook postInstall
          '';
        };

        packages.runner = pkgs.runCommand "yazi-picker-runner" { } ''
          mkdir -p $out/bin
          cp ${./pick.sh} $out/bin/pick
          chmod +x $out/bin/pick
        '';

        devShells.default = pkgs.mkShell {
          # inputsFrom = [ myPackage ];

          buildInputs = with pkgs; [
            cargo
            rustc

            rust-analyzer
            fenix.packages.${system}.latest.rustfmt

            python3
            python3Packages.dbus-python
          ];
          # RUSTFMT = "${fenix.packages.${system}.latest.rustfmt}/bin/rustfmt";
          # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
