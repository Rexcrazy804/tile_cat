{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... }: let
    forAllSystems = function: nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "aarch64-linux"
    ] (system: function nixpkgs.legacyPackages.${system});
  in {
    devShells = forAllSystems (pkgs: {
      default = with pkgs; mkShell {
        shellHook = ''
          exec $SHELL
        '';

        RUST_SRC_PATH = rustPlatform.rustLibSrc;
        RUST_BACKTACE = 1;

        LD_LIBRARY_PATH = "$LD_LIBRARY_PATH: ${lib.makeLibraryPath[
          udev
          alsaLib
          vulkan-loader
        ]}";

        nativeBuildInputs = [ clang pkg-config ];
        buildInputs = [
          cargo
          rustc
          rustfmt
          pre-commit
          rustPackages.clippy
          alsa-lib
          libudev-zero
          vulkan-loader 

          #NOTE Add more deps
          xorg.libX11
          xorg.libXrandr
          xorg.libXcursor
          xorg.libXi
        ];
      };
    });
  };
}
