use chain::Blockchain;
use chain::entity::Block;
use grpc;
use grpc::RequestOptions;
use grpc::SingleResponse;
use node::p2p::rpc::node_discovery_service::*;
use node::p2p::rpc::node_discovery_service_grpc::*;
use std::sync::Arc;

pub mod node_discovery_service;
pub mod node_discovery_service_grpc;

struct DiscoveryServiceImpl {
    blockchain: Arc<Blockchain>
}

impl node_discovery_service_grpc::DiscoveryService for DiscoveryServiceImpl {
    fn ping(&self, _opt: RequestOptions, _req: PingRequest) -> SingleResponse<PingResponse> {
        return SingleResponse::completed(PingResponse::new());
    }

    fn version(&self, _opt: RequestOptions, _req: VersionRequest) -> SingleResponse<VersionResponse> {
        let best_block = match self.blockchain.best_block() {
            Ok(block) => block,
            Err(_e) => return SingleResponse::err(grpc::Error::Panic("Not found".to_string())),
        };

        let response = VersionResponse::from(best_block);
        return SingleResponse::completed(response);
    }
}

impl From<Block> for VersionResponse {
    fn from(block: Block) -> VersionResponse {
        let mut response = VersionResponse::new();

        response.set_latestHash(block.header.hash.to_vec());
        response.set_latestIndex(block.header.index);

        response
    }
}

pub fn new_discovery_service_def(blockchain: Arc<Blockchain>) -> grpc::rt::ServerServiceDefinition {
    return DiscoveryServiceServer::new_service_def(DiscoveryServiceImpl { blockchain });
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ping() {
        use chain::Blockchain;
        use grpc;
        use node::Node;
        use node::NodeConfig;
        use node::p2p::rpc::node_discovery_service::*;
        use node::p2p::rpc::node_discovery_service_grpc::*;
        use rand::random;
        use std::sync::Arc;

        let port = random::<u16>();

        let blockchain = Arc::new(Blockchain::new());
        let _node = Node::start(blockchain, NodeConfig { port });

        let client = DiscoveryServiceClient::new_plain("localhost", port, Default::default()).unwrap();

        let response = client.ping(grpc::RequestOptions::new(), PingRequest::new());
        let result = response.wait();

        assert!(&result.is_ok());
    }

    #[test]
    fn test_version() {
        use chain::Blockchain;
        use grpc;
        use node::Node;
        use node::NodeConfig;
        use node::p2p::rpc::node_discovery_service::*;
        use node::p2p::rpc::node_discovery_service_grpc::*;
        use rand::random;
        use std::sync::Arc;

        let port = random::<u16>();

        let blockchain = Arc::new(Blockchain::new());
        let best_block = blockchain.best_block().expect("best block is required");
        let _node = Node::start(blockchain, NodeConfig { port });

        let client = DiscoveryServiceClient::new_plain("localhost", port, Default::default()).unwrap();

        let response = client.version(grpc::RequestOptions::new(), VersionRequest::new());
        let result = response.wait();

        assert!(&result.is_ok());
        let version_response = result.unwrap().1;

        assert_eq!(&best_block.header.hash[..], version_response.get_latestHash(), "expected best block hash to be equal");
        assert_eq!(best_block.header.index, version_response.get_latestIndex(), "expected best block index to be equal");
    }
}