{
  inputs = {
    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    nixpkgs = {
      url = "github:nixos/nixpkgs";
    };
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      {
        devShell =
          let
            pkgs = import nixpkgs {
              inherit system;
            };

          in
          pkgs.mkShell {
            buildInputs = with pkgs; [
              avrdude
              pkgsCross.avr.buildPackages.gcc
            ];
          };
      }
    )
  ;
}
