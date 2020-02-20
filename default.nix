with import ./nix { };
naersk.buildPackage {
  src = ./.;
  buildInputs = [ clang dbus.dev openssl.dev pkgconfig ];
  doCheck = false;
}
