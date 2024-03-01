load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

# See MODULE.bazel's additive_build_file tag.
# Extra build file content from file

load("@rules_rust//crate_universe:defs.bzl", "crates_vendor")

crates_vendor(
    name = "vendor",
    cargo_lockfile = "//:Cargo.lock",
    generate_build_scripts = True,
    manifests = ["//:Cargo.toml"],
    mode = "remote",
    tags = ["manual"],
)