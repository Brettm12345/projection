{ sources ? import ./sources.nix }:
let rust = import ./rust.nix { inherit sources; };
in import sources.nixpkgs {
  overlays = [
    (self: super:
      with self; rec {
        latest_rust = rust.rust;
        niv = (import sources.niv { }).niv;
        naersk = callPackage sources.naersk {
          rustc = latest_rust;
          cargo = latest_rust;
        };
      })
  ];
}
