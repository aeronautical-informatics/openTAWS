{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nmattia/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }:
    utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ]
      (system:
        let
          overlays = [ fenix.overlay ];
          pkgs = import nixpkgs { inherit system overlays; };
          fenix-toolchain = with fenix.packages.${system}; combine [
            default.rustc
            default.cargo
            default.clippy
            default.rust-src
            default.rustfmt
          ];
          naersk-lib = naersk.lib.${system}.override {
            inherit (fenix-toolchain);
          };
        in
        rec {
          packages.opentaws = naersk-lib.buildPackage {
            pname = "opentaws";
            root = ./.;
            doCheck = true;
            doDoc = true;
            copyLibs = true;
            doDocFail = true;
            copyTarget = true;
          };
          defaultPackage = packages.opentaws;

          devShell = pkgs.mkShell {
            nativeBuildInputs =
              [
                fenix-toolchain
                pkgs.jetbrains.idea-community
              ];
          };

          # Hail to the Hydra
          hydraJobs.opentaws."system" = packages.opentaws // {
            meta = {
              timeout = 86400;
              maxSilent = 36000;
            };
          };
        });
}
