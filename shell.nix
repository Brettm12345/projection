with import ./nix { };
stdenv.mkDerivation {
  name = "projection-shell-env";
  buildInputs = [
    clang
    cargo-bloat
    cargo-edit
    cargo-generate
    cargo-make
    cargo-sweep
    cargo-tree
    cargo-xbuild
    cargo-outdated
    cargo-release
    cargo-tree
    cargo-watch
    dbus.dev
    niv
    nixfmt
    racer
    pkgconfig
    openssl.dev
    rustup
    rustfmt
    latest_rust
  ];
  OPENSSL_DEV = openssl.dev;
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
}
