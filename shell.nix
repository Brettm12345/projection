let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  sources = import ./nix/sources.nix;
in with pkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    clang
    cargo
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
    latest.rustChannels.nightly.rust
  ];
  OPENSSL_DEV = openssl.dev;
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
}
