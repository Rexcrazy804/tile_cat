{pkgs, ...}: {
  projectRootFile = "./.git/config";
  programs.alejandra.enable = true;
  programs.rustfmt.enable = true;
}
