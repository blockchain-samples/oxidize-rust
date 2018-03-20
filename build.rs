extern crate protoc_rust_grpc;

fn main() {
    // blockchain_entities.proto
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/chain/wire",
        includes: &["_proto", "."],
        input: &["blockchain_entities.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");

    // blockchain_service.proto
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/chain/rpc",
        includes: &["_proto", "."],
        input: &["blockchain_service.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");

    // node_discovery_service.proto
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/node/p2p/rpc",
        includes: &["_proto", "."],
        input: &["node_discovery_service.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");

    // wallet_service.proto
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/wallet/rpc",
        includes: &["_proto", "."],
        input: &["wallet_service.proto"],
        rust_protobuf: true,
    }).expect("protoc-rust-grpc");
}