load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

rust_library(
    name = "rusty_raft",
    srcs = [
        "src/config.rs",
        "src/lib.rs",
        "src/log.rs",
        "src/node.rs",
        "src/raft.rs",
        "src/storage.rs",
        "src/proto/gen/raft_protobufs.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:tokio",
        "@crates//:serde",
        "@crates//:serde_json",
        "@crates//:prost",
        "@crates//:prost_types",
        "@crates//:tonic",
        "@crates//:log",
        "@crates//:env_logger",
        "@crates//:protobuf",
        "@crates//:rand_core",
    ],
)