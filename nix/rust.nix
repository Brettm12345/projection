{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
in pkgs.rustChannelOf {
  date = "2020-02-20";
  channel = "nightly";
}
