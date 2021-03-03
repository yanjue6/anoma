use std::path::PathBuf;

fn main() {
    // XXX TODO add header to file with "auto-generated"
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/proto/");
    tonic_build::configure()
        .out_dir(PathBuf::from("src/lib/protobuf"))
        .format(true)
         // XXX TODO can this be automatic for all type in a file ?
        .type_attribute("gossip.Intent", "#[derive(Hash)]")
        .type_attribute("gossip.Dkg", "#[derive(Hash)]")
        .compile(&["src/proto/service.proto"], &["src/proto"])
        .unwrap();
}
