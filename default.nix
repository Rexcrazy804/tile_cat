{
  sources ? import ./npins,
  pkgs ? import sources.nixpkgs {overlays = [(import sources.rust-overlay)];},
}: let
  inherit (pkgs.lib) fix makeScope;
  inherit (pkgs) newScope;

  rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
    extensions = ["rust-src" "rust-analyzer"];
    targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
  };
in
  fix (self: {
    packages = makeScope newScope (self': let
      inherit (self') callPackage;
      craneLib' = callPackage ((sources.crane {inherit pkgs;}) + "/lib") {};
    in {
      craneLib = craneLib'.overrideToolchain rustToolchain;
      tile-cat = callPackage ./package.nix {};
      default = self'.tile-cat;
    });

    devShells.default = let
      inherit (self.packages) craneLib tile-cat;
      devShell = craneLib.devShell.override (old: {
        mkShell = old.mkShell.override (old': {
          stdenv = pkgs.stdenvAdapters.useMoldLinker old'.stdenv;
        });
      });
    in
      devShell {
        inputsFrom = [tile-cat];
        # trunk build --public-url './' --release
        # pkgs.trunk
        packages = [];
      };
  })
