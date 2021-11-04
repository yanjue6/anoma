with import ./nix { };

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    rustNightly.rustfmt
    cargo-nightly
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
}
