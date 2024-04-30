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
      default = with pkgs; mkShell rec {
        shellHook = ''
          exec $SHELL
        '';

        RUST_SRC_PATH = rustPlatform.rustLibSrc;
        RUST_BACKTRACE = 1;
        WINIT_UNIX_BACKEND = "wayland";

        nativeBuildInputs = [ clang pkg-config ];

        bevyDependencies = [
            alsa-lib
            libudev-zero
            vulkan-loader 
            libxkbcommon
            wayland
        ];

        buildInputs = [
          cargo
          rustc
          rustfmt
          pre-commit
          rustPackages.clippy
          rust-analyzer
        ] ++ bevyDependencies;

        # Required for Bevy LD
        LD_LIBRARY_PATH = lib.makeLibraryPath bevyDependencies;
      };
    });
  };
}
