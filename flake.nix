{
  description = "Bearbroider Stack";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";
  
  inputs.turbo.url = "github:alexghr/turborepo.nix/v1.8.3";
  inputs.turbo.inputs.nixpkgs.follows = "nixpkgs";
  
  inputs.utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, turbo, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.bashInteractive ];
          buildInputs = [
            turbo.packages.${system}.default
            pkgs.nodejs
            # You can set the major version of Node.js to a specific one instead
            # of the default version
            # pkgs.nodejs-19_x

            # You can choose pnpm, yarn, or none (npm).
            pkgs.nodePackages.pnpm
            # pkgs.yarn

            pkgs.nodePackages.typescript
            pkgs.nodePackages.typescript-language-server
          ];
          shellHook = ''
            export TURBO_BINARY_PATH="${turbo.packages.${system}.default}/bin/turbo"
          '';
        };
      }
    );
}
