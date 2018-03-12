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

pub trait WalletService {
    fn account(&self, o: ::grpc::RequestOptions, p: super::wallet_service::AccountRequest) -> ::grpc::SingleResponse<super::wallet_service::AccountResponse>;

    fn unspent_outputs(&self, o: ::grpc::RequestOptions, p: super::wallet_service::UnspentOutputsRequest) -> ::grpc::SingleResponse<super::wallet_service::UnspentOutputsResponse>;

    fn propose_transaction(&self, o: ::grpc::RequestOptions, p: super::wallet_service::ProposeTransactionRequest) -> ::grpc::SingleResponse<super::wallet_service::ProposeTransactionResponse>;
}

// client

pub struct WalletServiceClient {
    grpc_client: ::grpc::Client,
    method_Account: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::wallet_service::AccountRequest, super::wallet_service::AccountResponse>>,
    method_UnspentOutputs: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::wallet_service::UnspentOutputsRequest, super::wallet_service::UnspentOutputsResponse>>,
    method_ProposeTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::wallet_service::ProposeTransactionRequest, super::wallet_service::ProposeTransactionResponse>>,
}

impl WalletServiceClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        WalletServiceClient {
            grpc_client: grpc_client,
            method_Account: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/rpc.WalletService/Account".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UnspentOutputs: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/rpc.WalletService/UnspentOutputs".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ProposeTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/rpc.WalletService/ProposeTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            WalletServiceClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            WalletServiceClient::with_client(c)
        })
    }
}

impl WalletService for WalletServiceClient {
    fn account(&self, o: ::grpc::RequestOptions, p: super::wallet_service::AccountRequest) -> ::grpc::SingleResponse<super::wallet_service::AccountResponse> {
        self.grpc_client.call_unary(o, p, self.method_Account.clone())
    }

    fn unspent_outputs(&self, o: ::grpc::RequestOptions, p: super::wallet_service::UnspentOutputsRequest) -> ::grpc::SingleResponse<super::wallet_service::UnspentOutputsResponse> {
        self.grpc_client.call_unary(o, p, self.method_UnspentOutputs.clone())
    }

    fn propose_transaction(&self, o: ::grpc::RequestOptions, p: super::wallet_service::ProposeTransactionRequest) -> ::grpc::SingleResponse<super::wallet_service::ProposeTransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_ProposeTransaction.clone())
    }
}

// server

pub struct WalletServiceServer;


impl WalletServiceServer {
    pub fn new_service_def<H : WalletService + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/rpc.WalletService",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/rpc.WalletService/Account".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.account(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/rpc.WalletService/UnspentOutputs".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.unspent_outputs(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/rpc.WalletService/ProposeTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.propose_transaction(o, p))
                    },
                ),
            ],
        )
    }
}
