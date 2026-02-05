{
  lib,
  rustPlatform,
  pkg-config,
  ...
}:

let
  portal_path = "org.freedesktop.impl.portal.desktop.rust_backend";
  manifest = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage rec {
  pname = manifest.package.name;
  version = manifest.package.version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];

  postInstall = ''
    mkdir -p $out/share/xdg-desktop-portal/portals
    mkdir -p $out/share/dbus-1/services
    mkdir -p $out/lib/systemd/user

    substitute yazi-picker.portal.in \
      $out/share/xdg-desktop-portal/portals/yazi-picker.portal \
      --replace "@PORTAL_NAME@" "${portal_path}" \
      --replace "@MAIN_BIN@" $out/bin/${pname}


    substitute yazi-picker.dbus.service.in \
      $out/share/dbus-1/services/${portal_path}.service \
      --replace "@PORTAL_NAME@" "${portal_path}" \
      --replace "@MAIN_BIN@" $out/bin/${pname}

    substitute yazi-picker.sysd.service.in \
      $out/lib/systemd/user/yazi-picker.service \
      --replace "@PORTAL_NAME@" "${portal_path}" \
      --replace "@MAIN_BIN@" $out/bin/${pname}
  '';
}
