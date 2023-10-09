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
        devShells.default = with pkgs; mkShell rec {
          nativeBuildInputs = [ bashInteractive ];
          buildInputs = [
            # Monorepo build system
            turbo.packages.${system}.default
            nodejs
            nodePackages.pnpm
            nodePackages.typescript
            nodePackages.typescript-language-server

            # Bevy
            openssl binaryen pkg-config
            udev alsa-lib vulkan-loader
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
            # libxkbcommon wayland # To use the wayland feature
          ];
          shellHook = ''
            export TURBO_BINARY_PATH="${turbo.packages.${system}.default}/bin/turbo"
          '';
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
