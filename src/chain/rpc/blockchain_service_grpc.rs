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

pub trait SyncService {
    fn get_best_header(&self, o: ::grpc::RequestOptions, p: super::blockchain_service::GetBestHeaderRequest) -> ::grpc::SingleResponse<super::blockchain_service::GetBestHeaderResponse>;

    fn get_headers(&self, o: ::grpc::RequestOptions, p: super::blockchain_service::GetHeadersRequest) -> ::grpc::SingleResponse<super::blockchain_service::GetHeadersResponse>;

    fn get_block(&self, o: ::grpc::RequestOptions, p: super::blockchain_service::GetBlockRequest) -> ::grpc::SingleResponse<super::blockchain_service::GetBlockResponse>;
}

// client

pub struct SyncServiceClient {
    grpc_client: ::grpc::Client,
    method_GetBestHeader: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain_service::GetBestHeaderRequest, super::blockchain_service::GetBestHeaderResponse>>,
    method_GetHeaders: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain_service::GetHeadersRequest, super::blockchain_service::GetHeadersResponse>>,
    method_GetBlock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::blockchain_service::GetBlockRequest, super::blockchain_service::GetBlockResponse>>,
}

impl SyncServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        SyncServiceClient {
            grpc_client: grpc_client,
            method_GetBestHeader: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/blockrpc.SyncService/GetBestHeader".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetHeaders: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/blockrpc.SyncService/GetHeaders".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetBlock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/blockrpc.SyncService/GetBlock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            SyncServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            SyncServiceClient::with_client(c)
        })
    }
}

impl SyncService for SyncServiceClient {
    fn get_best_header(&self, o: ::grpc::RequestOptions, p: super::blockchain_service::GetBestHeaderRequest) -> ::grpc::SingleResponse<super::blockchain_service::GetBestHeaderResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetBestHeader.clone())
    }

    fn get_headers(&self, o: ::grpc::RequestOptions, p: super::blockchain_service::GetHeadersRequest) -> ::grpc::SingleResponse<super::blockchain_service::GetHeadersResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetHeaders.clone())
    }

    fn get_block(&self, o: ::grpc::RequestOptions, p: super::blockchain_service::GetBlockRequest) -> ::grpc::SingleResponse<super::blockchain_service::GetBlockResponse> {
        self.grpc_client.call_unary(o, p, self.method_GetBlock.clone())
    }
}

// server

pub struct SyncServiceServer;


impl SyncServiceServer {
    pub fn new_service_def<H : SyncService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/blockrpc.SyncService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/blockrpc.SyncService/GetBestHeader".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_best_header(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/blockrpc.SyncService/GetHeaders".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_headers(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/blockrpc.SyncService/GetBlock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_block(o, p))
                    },
                ),
            ],
        )
    }
}
