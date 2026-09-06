[package]
name = "srbehunin-proto"
version = "0.2.0"
edition = "2024"
description = "Protobuf message types and gRPC service stubs for SRBehunin."
license = "MIT"
repository = "https://github.com/srbehunin/srbehunin-proto/"

[dependencies]
prost = "0.13"
tonic = "0.12"
serde = { version = "1", features = ["derive"] }

[features]
@@protoc_insertion_point(features)
