{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ]
    (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        naersk-lib = naersk.lib."${system}";
      in rec {
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
          nativeBuildInputs = with pkgs;
            [
              (rust-bin.stable.latest.default.override {
                extensions =
                  [ "rust-src" "clippy" "rustfmt" "llvm-tools-preview" ];
                targets = [ "arm-unknown-linux-gnueabihf" ];
              })
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
