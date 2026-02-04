{
  lib,
  rustPlatform,
  pkg-config,
  ...
}:

let
  # 1. Import the Cargo.toml file as a Nix set
  manifest = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage {
  # 2. Extract name and version dynamically
  pname = manifest.package.name;
  version = manifest.package.version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];


  # (Rest of your postInstall logic remains the same)
  postInstall = ''
    # mkdir -p $out/share/dbus-1/services
    # mkdir -p $out/lib/systemd/user

    # cp org.freedesktop.impl.portal.desktop.MyService.service.in \
    #    $out/share/dbus-1/services/org.freedesktop.impl.portal.desktop.MyService.service

    # cp my-custom-picker.service.in \
    #    $out/lib/systemd/user/my-custom-picker.service

    # substituteInPlace $out/share/dbus-1/services/org.freedesktop.impl.portal.desktop.MyService.service \
    #    --replace "@BIN_DIR@" "$out/bin"

    # substituteInPlace $out/lib/systemd/user/my-custom-picker.service \
    #    --replace "@BIN_DIR@" "$out/bin"
  '';
}
