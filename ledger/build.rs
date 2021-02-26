use std::env;
use std::path::PathBuf;
use tonic_build;

fn main() {
    tonic_build::configure()
        .out_dir(PathBuf::from("src/lib/protobuf"))
        .compile(&["src/proto/gossip.proto"], &["src/proto"])
        .unwrap();
}