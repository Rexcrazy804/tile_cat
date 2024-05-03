{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt.url = "github:numtide/treefmt-nix";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    treefmt,
    ...
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (
        system:
          function
          (import nixpkgs {
            inherit system;
            overlays = [rust-overlay.overlays.default];
          })
      );
    treefmtEval = forAllSystems (pkgs: treefmt.lib.evalModule pkgs ./treefmt.nix);
  in {
    formatter = forAllSystems (
      pkgs:
        treefmtEval.${pkgs.system}.config.build.wrapper
    );
    checks = forAllSystems (pkgs: {
      formatting = treefmtEval.${pkgs.system}.config.build.check self;
    });

    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell.override {
        stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
      } rec {
        shellHook = ''
          exec $SHELL
        '';

        RUST_BACKTRACE = 1;

        nativeBuildInputs = with pkgs; [
          (rust-bin.nightly.latest.default.override {
            targets = ["wasm32-unknown-unknown"];
          })
        ];

        buildInputs = with pkgs; [
          pkg-config
          alsa-lib
          libudev-zero
          vulkan-loader
          libxkbcommon
          wayland
        ];

        packages = [pkgs.rust-analyzer];

        # Required for Bevy LD
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
      };
    });
  };
}
