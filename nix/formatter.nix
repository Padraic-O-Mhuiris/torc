{ config, inputs, ... }:

{
  config.perSystem = { pkgs, ... }: {
    treefmt.config = {
      projectRootFile = "flake.nix";
      programs = {
        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
        taplo.enable = true;
      };
    };
  };
}
