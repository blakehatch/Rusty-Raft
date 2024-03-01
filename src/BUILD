load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

rust_library(
    name = "rusty_raft",
    srcs = [
        "config.rs",
        "lib.rs",
        "log.rs",
        "node.rs",
        "raft.rs",
        "storage.rs",
        "raft_protobufs.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:tokio",
        "@crates//:serde",
        "@crates//:serde_json",
        "@crates//:prost",
        "@crates//:prost-types",
        "@crates//:tonic",
        "@crates//:log",
        "@crates//:env_logger",
        "@crates//:protobuf",
        "@crates//:rand_core",
    ],
)