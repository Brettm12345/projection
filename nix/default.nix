{ sources ? import ./sources.nix }:
import sources.nixpkgs {
  overlays = [
    (import sources.nixpkgs-mozilla)
    (self: super:
      with self; {
        niv = (import sources.niv { }).niv;
        naersk = callPackage sources.naersk { };
      })
  ];
}
