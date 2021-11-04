{ rustOverlay ? import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")
# rustc from rust-toolchain.toml
, rustChannel ? (builtins.fromTOML (builtins.readFile ../rust-toolchain.toml)).toolchain.channel
# rustfmt, clippy, miri from nightly
, rustNightlyVersion ? builtins.substring 8 10 (builtins.readFile ../rust-nightly-version)
}:

import <nixpkgs> {
  overlays = [
    rustOverlay

    # Rust toolchains
    (self: super: {
        rustc = self.rust-bin.stable.${rustChannel}.minimal.override {
          targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
        };

        rustNightly = self.rust-bin.nightly.${rustNightlyVersion};

        cargo-nightly = self.runCommandNoCC "cargo-nightly" { buildInputs = [ self.makeWrapper ]; } ''
          mkdir -p $out/bin
          makeWrapper ${self.rustNightly.cargo}/bin/cargo $out/bin/cargo-nightly \
            --prefix PATH : ${self.lib.makeBinPath [
              (self.rustNightly.default.override {
                targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
              })

              # XXX cargo-miri appears to be too heavily integrated with xargo/rustup. Idk how to make it work properly under nix.
              self.rustNightly.miri
            ]}
        '';
      }
    )

    # Other 3rd party packages
    (self: super: {
      mdbook-linkcheck = self.callPackage ./mdbook-linkcheck.nix { };
    })
  ];
}
