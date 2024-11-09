{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };

  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs = { self, nixpkgs, devenv, ... } @ inputs:
    let
      system = "x86_64-linux";

      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.devenv-up = self.devShells.${system}.default.config.procfileScript;

      devShells.${system}.default = devenv.lib.mkShell {
        inherit inputs pkgs;
        modules = [
          ({ pkgs, config, ... }: {
            # https://devenv.sh/packages/
            packages = with pkgs;[
              git
              clang
              libclang
              libvncserver
              cmake
            ];

            enterShell = ''
              export LIBCLANG_PATH="${pkgs.libclang.lib}/lib"
            '';

            # https://devenv.sh/tests/
            enterTest = ''

            '';

            # https://devenv.sh/services/
            # services.postgres.enable = true;

            # https://devenv.sh/languages/
            languages = {
              rust = {
                enable = true;
                channel = "nightly";
              };
              c.enable = true;
            };

            # https://devenv.sh/pre-commit-hooks/
            # pre-commit.hooks.shellcheck.enable = true;

            # https://devenv.sh/processes/
            # processes.ping.exec = "ping example.com";

            # See full reference at https://devenv.sh/reference/options/

          })
        ];
      };
    };
}
