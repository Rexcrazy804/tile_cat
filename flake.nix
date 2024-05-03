{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (system: 
        function 
          (import nixpkgs { 
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          })
      );
  in {
    formatter = forAllSystems (pkgs: pkgs.alejandra);
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell rec {
          shellHook = ''
            exec $SHELL
          '';

          RUST_BACKTRACE = 1;

          nativeBuildInputs = with pkgs; [
            clang
            (rust-bin.nightly.latest.default.override {
              targets = [ "wasm32-unknown-unknown" ];
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

          packages = [ pkgs.rust-analyzer ];

          # Required for Bevy LD
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
    });
  };
}
