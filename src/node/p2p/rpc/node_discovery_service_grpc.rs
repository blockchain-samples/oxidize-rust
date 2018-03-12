// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait DiscoveryService {
    fn ping(&self, o: ::grpc::RequestOptions, p: super::node_discovery_service::PingRequest) -> ::grpc::SingleResponse<super::node_discovery_service::PingResponse>;

    fn version(&self, o: ::grpc::RequestOptions, p: super::node_discovery_service::VersionRequest) -> ::grpc::SingleResponse<super::node_discovery_service::VersionResponse>;
}

// client

pub struct DiscoveryServiceClient {
    grpc_client: ::grpc::Client,
    method_Ping: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node_discovery_service::PingRequest, super::node_discovery_service::PingResponse>>,
    method_Version: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::node_discovery_service::VersionRequest, super::node_discovery_service::VersionResponse>>,
}

impl DiscoveryServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        DiscoveryServiceClient {
            grpc_client: grpc_client,
            method_Ping: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/p2p.DiscoveryService/Ping".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Version: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/p2p.DiscoveryService/Version".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            DiscoveryServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            DiscoveryServiceClient::with_client(c)
        })
    }
}

impl DiscoveryService for DiscoveryServiceClient {
    fn ping(&self, o: ::grpc::RequestOptions, p: super::node_discovery_service::PingRequest) -> ::grpc::SingleResponse<super::node_discovery_service::PingResponse> {
        self.grpc_client.call_unary(o, p, self.method_Ping.clone())
    }

    fn version(&self, o: ::grpc::RequestOptions, p: super::node_discovery_service::VersionRequest) -> ::grpc::SingleResponse<super::node_discovery_service::VersionResponse> {
        self.grpc_client.call_unary(o, p, self.method_Version.clone())
    }
}

// server

pub struct DiscoveryServiceServer;


impl DiscoveryServiceServer {
    pub fn new_service_def<H : DiscoveryService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/p2p.DiscoveryService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/p2p.DiscoveryService/Ping".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.ping(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/p2p.DiscoveryService/Version".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.version(o, p))
                    },
                ),
            ],
        )
    }
}
