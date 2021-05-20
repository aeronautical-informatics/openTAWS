{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
  utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ] (system:
  let
    naersk-lib = naersk.lib."${system}";
    overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
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
      buildInputs = [
        pkgs.openssl
        pkgs.pkgconfig
        pkgs.exa
        pkgs.fd
        (pkgs.rust-bin.stable.latest.default.override { extensions = [
          "rust-src" ]; targets = [ "arm-unknown-linux-gnueabihf" "thumbv7em-none-eabi" ]; })
      ];
      shellHook = ''
        alias ls=exa
        alias find=fd
      '';
    };

    # Hail to the Hydra
    hydraJobs = packages;
  });
}
