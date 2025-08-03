# this currently only has a singular devshell
# but for future reference if we require mutlitple devshells
# we can call nix-shell --argstr shell "<shellname>"
{shell ? "default"}: (import ./. {}).devShells.${shell}
