use chain::Blockchain;
use chain::entity::Block;
use chain::entity::Header;
use chain::rpc::blockchain_service::*;
use chain::rpc::blockchain_service_grpc::*;
use chain::wire::*;
use chain::wire::blockchain_entities;
use futures::Future;
use grpc;
use grpc::RequestOptions;
use grpc::SingleResponse;
use hash::Hash;
use hash::hash_from_bytes;
use std::sync::Arc;

mod blockchain_service;
mod blockchain_service_grpc;

pub struct SyncClient {
    client: SyncServiceClient
}

impl SyncClient {
    pub fn new(host: &str, port: u16) -> grpc::Result<SyncClient> {
        SyncServiceClient::new_plain(host, port, grpc::ClientConf::new())
            .map(|c| SyncClient { client: c })
    }

    pub fn get_best_header(&self) -> Box<Future<Item=Header, Error=grpc::Error>> {
        let best_header = self.client.get_best_header(RequestOptions::new(), GetBestHeaderRequest::new())
            .drop_metadata()
            .map(|res| from_wire_header(res.get_header().clone()));

        return Box::new(best_header);
    }

    pub fn get_headers(&self, latest_index: u64, latest_hash: Hash) -> Box<Future<Item=Vec<Header>, Error=grpc::Error>> {
        let mut request = GetHeadersRequest::new();
        request.set_latestIndex(latest_index);
        request.set_latestHash(latest_hash.to_vec());

        let headers = self.client.get_headers(RequestOptions::new(), request)
            .drop_metadata()
            .map(|res| res.get_headers().iter()
                .map(|header| from_wire_header(header.clone()))
                .collect()
            );

        return Box::new(headers);
    }

    pub fn get_block(&self, index: u64, hash: Hash) -> Box<Future<Item=Block, Error=grpc::Error>> {
        let mut request = GetBlockRequest::new();
        request.set_index(index);
        request.set_hash(hash.to_vec());

        let block = self.client.get_block(RequestOptions::new(), request)
            .drop_metadata()
            .map(|res| from_wire_block(res.get_block().clone()));

        return Box::new(block);
    }
}

struct SyncServiceImpl {
    blockchain: Arc<Blockchain>
}

impl blockchain_service_grpc::SyncService for SyncServiceImpl {
    fn get_best_header(&self, _opt: RequestOptions, _req: GetBestHeaderRequest) -> SingleResponse<GetBestHeaderResponse> {
        return self.blockchain.best_header()
            .map(|option| {
                option.map_or(
                    SingleResponse::err(grpc::Error::Panic("Not found".to_string())),
                    |header| SingleResponse::completed(to_get_best_header_response(header)),
                )
            })
            .unwrap_or_else(|e| SingleResponse::err(grpc::Error::Panic(e.to_string())));
    }

    fn get_headers(&self, _opt: RequestOptions, _req: GetHeadersRequest) -> SingleResponse<GetHeadersResponse> {
        return self.blockchain.get_headers()
            .map(|headers|
                headers.iter()
                    .map(|header| to_wire_header(header.clone()))
                    .collect()
            )
            .map(|headers| SingleResponse::completed(to_get_headers_response(headers)))
            .unwrap_or_else(|e| SingleResponse::err(grpc::Error::Panic(e.to_string())));
    }

    fn get_block(&self, _opt: RequestOptions, req: GetBlockRequest) -> SingleResponse<GetBlockResponse> {
        return self.blockchain.get_block_by_hash(hash_from_bytes(req.get_hash()))
            .map(|option| {
                option.map_or(
                    SingleResponse::err(grpc::Error::Panic("Not found".to_string())),
                    |block| SingleResponse::completed(to_get_block_response(block)),
                )
            })
            .unwrap_or_else(|e| SingleResponse::err(grpc::Error::Panic(e.to_string())));
    }
}

pub fn new_sync_service_def(blockchain: Arc<Blockchain>) -> grpc::rt::ServerServiceDefinition {
    return SyncServiceServer::new_service_def(SyncServiceImpl { blockchain });
}

fn to_get_best_header_response(header: Header) -> GetBestHeaderResponse {
    let mut res = GetBestHeaderResponse::new();
    res.set_header(to_wire_header(header));

    return res;
}

fn to_get_headers_response(headers: Vec<::chain::wire::BlockHeader>) -> GetHeadersResponse {
    let mut res = GetHeadersResponse::new();
    res.set_headers(::protobuf::RepeatedField::from_vec(headers));
    res.set_headerCount(0);

    return res;
}

fn to_get_block_response(block: Block) -> GetBlockResponse {
    let mut res = GetBlockResponse::new();
    res.set_block(to_wire_block(block));

    return res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_best_header() {
        use chain::Blockchain;
        use futures::Future;
        use std::sync::Arc;

        let blockchain = Arc::new(Blockchain::new());
        let best_header = blockchain.best_header()
            .expect("best header is required")
            .expect("best header is required");
        let (_node, client) = setup(blockchain);

        let response = client.get_best_header();
        let result = response.wait();

        let header = result.expect("expected no error");
        assert_eq!(header, best_header, "expected best header to be equal");
    }

    #[test]
    fn test_get_headers() {
        use chain::Blockchain;
        use futures::Future;
        use std::sync::Arc;

        let blockchain = Arc::new(Blockchain::new());
        let expected_headers = blockchain.get_headers().expect("headers are required");
        let (_node, client) = setup(blockchain);

        let response = client.get_headers(0, ::hash::EMPTY_HASH);
        let result = response.wait();

        let headers = result.expect("expected no error");
        assert_eq!(expected_headers, headers, "expected headers to be equal");
    }

    #[test]
    fn test_get_block() {
        use chain::Blockchain;
        use futures::Future;
        use std::sync::Arc;
        use chain::entity::genesis_hash;

        let blockchain = Arc::new(Blockchain::new());
        let (_node, client) = setup(blockchain);

        let response = client.get_block(0, genesis_hash());
        let result = response.wait();

        result.expect("expected no error");
    }

    fn setup(blockchain: ::std::sync::Arc<::chain::Blockchain>) -> (::node::Node, super::SyncClient) {
        use node::Node;
        use node::NodeConfig;
        use chain::rpc::SyncClient;
        use std::borrow::Borrow;

        let node = Node::start(blockchain, NodeConfig { addr: None });
        let client = SyncClient::new(node.local_addr().ip().to_string().borrow(), node.local_addr().port()).unwrap();

        return (node, client);
    }
}