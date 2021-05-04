{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk}:
  utils.lib.eachSystem [ "aarch64-linux" "i686-linux" "x86_64-linux" ] (system:
  let
    pkgs = nixpkgs.legacyPackages."${system}";
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
      nativeBuildInputs = with pkgs; [ rustc cargo ];
    };

    # Hail to the Hydra
    hydraJobs = packages;
  });
}
