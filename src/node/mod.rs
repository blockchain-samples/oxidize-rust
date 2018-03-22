use chain;
use chain::Blockchain;
use grpc::Error;
use grpc::Server;
use grpc::ServerBuilder;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod p2p;

pub struct NodeConfig<'a> {
    pub addr: Option<(&'a str, u16)>,
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

    pub fn local_addr(&self) -> &SocketAddr {
        self.rcp_server.local_addr()
    }

    pub fn is_alive(&self) -> bool {
        self.rcp_server.is_alive()
    }
}

fn new_rpc_server(blockchain: Arc<Blockchain>, node_config: NodeConfig) -> Result<Server, Error> {
    let mut server = ServerBuilder::new_plain();
    server.http.set_addr(node_config.addr.unwrap_or(("127.0.0.1", 0)))?;

    server.add_service(p2p::rpc::new_discovery_service_def(Arc::clone(&blockchain)));
    server.add_service(chain::rpc::new_sync_service_def(blockchain));

    server.build()
}