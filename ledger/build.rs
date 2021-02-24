use std::env;
use std::path::PathBuf;
use tonic_build;

fn main() {
    tonic_build::compile_protos("src/proto/rpc.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}
