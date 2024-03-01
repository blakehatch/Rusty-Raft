use std::fs;
use std::path::Path;

fn main() {
    let proto_file = "./src/proto/raft.proto";

    let src = Path::new("src/proto/gen/raft.protobufs.rs");
    let dst = Path::new("src/raft_protobufs.rs");

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/proto/gen")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    if src.exists() {
        fs::copy(src, dst).expect("Failed to copy protobufs file");
        println!("cargo:rerun-if-changed=src/proto/gen/raft.protobufs.rs");
    }

    println!("cargo:rerun-if-changed={}", proto_file);
}