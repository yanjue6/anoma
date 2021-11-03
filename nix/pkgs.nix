{ rustOverlay ? import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz")
# rustc from rust-toolchain.toml
, rustChannel ? (builtins.fromTOML (builtins.readFile ../rust-toolchain.toml)).toolchain.channel
# rustfmt, clippy, miri from nightly
, rustNightlyVersion ? builtins.substring 8 10 (builtins.readFile ../rust-nightly-version)
}:

import <nixpkgs> {
  overlays = [
    rustOverlay

    (self: super: {
        rustc = self.rust-bin.stable.${rustChannel}.minimal.override {
          targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" ];
        };
        inherit (self.rust-bin.nightly.${rustNightlyVersion}) rustfmt clippy miri;
      }
    )

    # Other 3rd party packages
    (self: super: {
      mdbook-linkcheck = self.callPackage ./mdbook-linkcheck.nix { };
    })
  ];
}
