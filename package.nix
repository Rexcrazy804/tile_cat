{
  darwin,
  stdenv,
  lib,
  craneLib,
  pkg-config,
  makeWrapper,
  zstd,
  alsa-lib,
  libxkbcommon,
  udev,
  vulkan-loader,
  wayland,
  xorg,
  rustPlatform,
}: let
  inherit (lib) optionals makeLibraryPath;
  inherit (craneLib) cleanCargoSource buildDepsOnly buildPackage;
  src = cleanCargoSource ./.;
  commonArgs = {
    inherit src;
    strictDeps = true;
    nativeBuildInputs = [pkg-config makeWrapper];
    buildInputs =
      [zstd]
      ++ (
        optionals stdenv.isLinux [
          alsa-lib
          libxkbcommon
          udev
          vulkan-loader
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ]
      )
      ++ (
        optionals stdenv.isDarwin [
          darwin.apple_sdk_11_0.frameworks.Cocoa
          rustPlatform.bindgenHook
        ]
      );
  };
  cargoArtifacts = buildDepsOnly commonArgs;
in
  buildPackage (
    commonArgs
    // {
      inherit cargoArtifacts;

      env = {
        ZSTD_SYS_USE_PKG_CONFIG = true;
      };

      postInstall =
        ''
          wrapProgram $out/bin/tile_cat \
            --prefix LD_LIBRARY_PATH : ${makeLibraryPath commonArgs.buildInputs}
        ''
        + (
          lib.optionalString stdenv.hostPlatform.isLinux ''
            patchelf $out/bin/.tile_cat-wrapped \
              --add-rpath ${makeLibraryPath [vulkan-loader]}
          ''
        );

      postFixup = ''
        mkdir -p $out/share
        cp -r ${./assets} $out/share/assets/
        ln -sf $out/share/assets $out/bin/assets
      '';
    }
  )
