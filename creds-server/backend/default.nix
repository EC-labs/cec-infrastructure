{ 
    pkgs ? import <nixpkgs> {},
    crate2nixTools
}:
let 
    crateOverrides = { };
    customBuildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
        defaultCrateOverrides = pkgs.defaultCrateOverrides // crateOverrides;
    };
    generatedCargoNix = import (
        (pkgs.callPackage crate2nixTools {}).generatedCargoNix {
          src = ./.;
          name = "cec-crate";
        }
    );
in pkgs.callPackage generatedCargoNix {
    buildRustCrateForPkgs = customBuildRustCrateForPkgs;
}
