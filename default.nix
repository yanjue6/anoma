{ pkgs ? import ./nix/pkgs.nix { }
}:

rec {
  # Generate Cargo.nix with: crate2nix generate
  cargo_nix = import ./Cargo.nix {
    inherit pkgs;
    buildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
      defaultCrateOverrides =
        pkgs.defaultCrateOverrides // import nix/crate-overrides.nix pkgs;
    };
  };

  apps = cargo_nix.workspaceMembers.anoma_apps;

  docs = import ./docs { inherit pkgs; };

  devShell = pkgs.mkShell {
    buildInputs = with pkgs; [
      rustc
      rustfmt
      clippy
      miri
      clang
      llvmPackages.libclang
      protobuf
      crate2nix
      # Needed at runtime
      tendermint
    ] ++ lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ];

    inputsFrom = [ docs ];

    LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
    PROTOC = "${pkgs.protobuf}/bin/protoc";
    PROTOC_INCLUDE = "${pkgs.protobuf}/include";
  };
}
