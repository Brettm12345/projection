with import ./nix { };
naersk.buildPackage {
  src = ./.;
  noCheck = true;
  buildInputs = [ clang dbus.dev openssl.dev pkgconfig ];
}
