{
  inputs = {
    utils.url = "git+https://github.com/numtide/flake-utils.git";
    fenix = {
      url = "git+https://github.com/nix-community/fenix.git?ref=main";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "git+https://github.com/nix-community/naersk.git";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, fenix, naersk }:
    utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ]
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
          rust-toolchain = with fenix.packages.${system}; combine [
            rust-analyzer
            stable.cargo
            stable.rustc
            stable.rustfmt
            targets.armv7a-none-eabi.stable.rust-std
          ];
          naersk-lib = (naersk.lib."${system}".override {
            cargo = rust-toolchain;
            rustc = rust-toolchain;
          });
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
            inputsFrom = [ packages.opentaws ];
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
