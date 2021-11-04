{ pkgs ? import ./pkgs.nix { }
}:

rec {
  inherit pkgs;

  # you should generate Cargo.nix with "crate2nix generate" since it is not checked in to the repo
  cargo_nix = import ../Cargo.nix {
    inherit pkgs;
    buildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
      defaultCrateOverrides =
        pkgs.defaultCrateOverrides // import ./crate-overrides.nix pkgs;
    };
  };

  apps = cargo_nix.workspaceMembers.anoma_apps.build;

  docs = import ../docs { inherit pkgs; };
}
