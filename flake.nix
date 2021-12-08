{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, utils, naersk, fenix }:
    utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ]
      (system:
        let
          overlays = [ fenix.overlay ];
          pkgs = import nixpkgs { inherit system overlays; };
          lib = pkgs.lib;
          fenix-toolchain = with fenix.packages.${system}; combine [
            default.cargo
            default.clippy
            default.rustc
            default.rustfmt
          ];
          naersk-lib = naersk.lib.${system}.override {
            inherit (fenix-toolchain);
          };
          AIRPORTS_JSON_FILE = builtins.fetchurl {
            url = "https://datahub.io/core/airport-codes/r/airport-codes.json";
            sha256 = "0006psrqcc121x3k8ffjlf7fgkc1r05dhv3964hsa3w3nlqsad8w";
          };
        in
        rec {
          # nix build
          packages.opentaws = naersk-lib.buildPackage {
            pname = "opentaws";
            root = ./.;
            doCheck = true;
            doDoc = true;
            copyLibs = true;
            doDocFail = true;
            copyTarget = true;
            overrideMain = (_: { inherit AIRPORTS_JSON_FILE; });
          };
          defaultPackage = packages.opentaws;

          # nix develop
          devShell = pkgs.mkShell {
            inherit AIRPORTS_JSON_FILE;
            nativeBuildInputs = [ fenix-toolchain pkgs.jetbrains.idea-community ];
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

