use std::path::PathBuf;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/proto/");
    tonic_build::configure()
        .out_dir(PathBuf::from("src/lib/protobuf"))
        .format(true)
        .compile(&["src/proto/service.proto"], &["src/proto"])
        .unwrap();
}
