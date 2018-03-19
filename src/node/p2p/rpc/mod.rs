use chain::Blockchain;
use chain::entity::Block;
use futures::Future;
use grpc;
use grpc::RequestOptions;
use grpc::SingleResponse;
use node::p2p::rpc::node_discovery_service::*;
use node::p2p::rpc::node_discovery_service_grpc::*;
use std::sync::Arc;

mod node_discovery_service;
mod node_discovery_service_grpc;

pub struct DiscoveryClient {
    client: DiscoveryServiceClient
}

impl DiscoveryClient {
    fn new(host: &str, port: u16) -> grpc::Result<DiscoveryClient> {
        DiscoveryServiceClient::new_plain(host, port, ::grpc::ClientConf::new())
            .map(|c| DiscoveryClient { client: c })
    }

    fn ping(&self) -> Box<Future<Item=(), Error=grpc::Error>> {
        let ping = self.client.ping(grpc::RequestOptions::new(), PingRequest::new())
            .drop_metadata()
            .map(|_res| ());

        return Box::new(ping);
    }

    fn version(&self) -> Box<Future<Item=(u64, ::hash::Hash), Error=grpc::Error>> {
        let version = self.client.version(grpc::RequestOptions::new(), VersionRequest::new())
            .drop_metadata()
            .map(|res| {
                (res.get_latestIndex(), ::hash::hash_from_bytes(res.get_latestHash()))
            });
        return Box::new(version);
    }
}

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

        return SingleResponse::completed(VersionResponse::from(best_block));
    }
}

impl From<Block> for VersionResponse {
    fn from(block: Block) -> VersionResponse {
        let mut response = VersionResponse::new();

        response.set_latestHash(block.header.hash.to_vec());
        response.set_latestIndex(block.header.index);

        return response;
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
        use std::sync::Arc;
        use futures::Future;

        let blockchain = Arc::new(Blockchain::new());
        let (_node, client) = setup(blockchain);

        let response = client.ping();
        let result = response.wait();

        assert!(&result.is_ok());
    }

    #[test]
    fn test_version() {
        use chain::Blockchain;
        use futures::Future;
        use std::sync::Arc;

        let blockchain = Arc::new(Blockchain::new());
        let best_block = blockchain.best_block().expect("best block is required");
        let (_node, client) = setup(blockchain);

        let response = client.version();
        let result = response.wait();

        assert!(&result.is_ok());
        let (best_index, best_hash) = result.unwrap();

        assert_eq!(&best_block.header.hash[..], best_hash, "expected best block hash to be equal");
        assert_eq!(best_block.header.index, best_index, "expected best block index to be equal");
    }

    fn setup(blockchain: ::std::sync::Arc<::chain::Blockchain>) -> (::node::Node, super::DiscoveryClient) {
        use node::Node;
        use node::NodeConfig;
        use node::p2p::rpc::DiscoveryClient;
        use rand::random;

        let port = random::<u16>();
        let node = Node::start(blockchain, NodeConfig { port });
        let client = DiscoveryClient::new("localhost", port).unwrap();

        return (node, client);
    }
}