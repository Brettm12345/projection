{ sources ? import ./sources.nix }:
let rust = import ./rust.nix { inherit sources; };
in import sources.nixpkgs {
  overlays = [
    (self: super:
      with self; {
        niv = (import sources.niv { }).niv;
        naersk = callPackage sources.naersk {
          rustc = rust.rust;
          cargo = rust.rust;
        };
      })
  ];
}
