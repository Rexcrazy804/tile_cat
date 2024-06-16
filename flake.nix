{
  description = "A Flake for setting up Rust to work with bevy with wasm32 build target";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.treefmt.flakeModule
      ];

      flake = {
        # original stuff? idk what this does just yet
      };

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = ["rust-src"];
          targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
        };

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

        nativeBuildInputs = with pkgs; [ pkg-config makeWrapper ];

        buildInputs = [
          pkgs.zstd
        ] ++ (with pkgs; lib.optionals stdenv.isLinux [
          alsa-lib
          libxkbcommon
          udev
          vulkan-loader
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ]) ++ ( with pkgs; lib.optionals stdenv.isDarwin [
          darwin.apple_sdk_11_0.frameworks.Cocoa
          rustPlatform.bindgenHook
        ]);

        # Required for Bevy LD
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

        tile_cat = craneLib.buildPackage {
          src = ./.;
          strictDeps = true;
          doCheck = false;

          inherit nativeBuildInputs buildInputs;

          env = {
            ZSTD_SYS_USE_PKG_CONFIG = true;
          };

          postInstall = ''
            cp -r assets $out/bin
            wrapProgram $out/bin/tile_cat \
              --prefix LD_LIBRARY_PATH : ${LD_LIBRARY_PATH}
          '';
        };
      in {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };

        treefmt = {
          projectRootFile = "./.git/config";
          programs = {
            alejandra.enable = true;
            rustfmt = {
              enable = true;
              package = pkgs.rust-bin.nightly.latest.rustfmt;
            };
          };
        };

        packages.default = tile_cat;

        devShells.default = craneLib.devShell {
          inputsFrom = [ tile_cat ];
          packages = [
            # trunk build --public-url './' --release
            pkgs.trunk
          ];
        };
      };
    };
}
