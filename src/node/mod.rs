use chain::Blockchain;
use grpc::Error;
use grpc::Server;
use grpc::ServerBuilder;
use node::p2p::rpc::new_discovery_service_def;
use std::sync::Arc;

pub mod p2p;

pub struct NodeConfig {
    pub port: u16
}

pub struct Node {
    rcp_server: Server,
}

impl Node {
    pub fn start(blockchain: Arc<Blockchain>, node_config: NodeConfig) -> Node {
        let rcp_server = new_rpc_server(blockchain, node_config).expect("rpc server");

        Node {
            rcp_server,
        }
    }
}

fn new_rpc_server(blockchain: Arc<Blockchain>, node_config: NodeConfig) -> Result<Server, Error> {
    let mut server = ServerBuilder::new_plain();
    server.http.set_port(node_config.port);

    server.add_service(new_discovery_service_def(blockchain));

    server.build()
}