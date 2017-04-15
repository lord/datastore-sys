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

pub trait Datastore {
    fn Lookup(&self, p: super::datastore::LookupRequest) -> ::grpc::result::GrpcResult<super::datastore::LookupResponse>;

    fn RunQuery(&self, p: super::datastore::RunQueryRequest) -> ::grpc::result::GrpcResult<super::datastore::RunQueryResponse>;

    fn BeginTransaction(&self, p: super::datastore::BeginTransactionRequest) -> ::grpc::result::GrpcResult<super::datastore::BeginTransactionResponse>;

    fn Commit(&self, p: super::datastore::CommitRequest) -> ::grpc::result::GrpcResult<super::datastore::CommitResponse>;

    fn Rollback(&self, p: super::datastore::RollbackRequest) -> ::grpc::result::GrpcResult<super::datastore::RollbackResponse>;

    fn AllocateIds(&self, p: super::datastore::AllocateIdsRequest) -> ::grpc::result::GrpcResult<super::datastore::AllocateIdsResponse>;
}

pub trait DatastoreAsync {
    fn Lookup(&self, p: super::datastore::LookupRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::LookupResponse>;

    fn RunQuery(&self, p: super::datastore::RunQueryRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::RunQueryResponse>;

    fn BeginTransaction(&self, p: super::datastore::BeginTransactionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::BeginTransactionResponse>;

    fn Commit(&self, p: super::datastore::CommitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::CommitResponse>;

    fn Rollback(&self, p: super::datastore::RollbackRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::RollbackResponse>;

    fn AllocateIds(&self, p: super::datastore::AllocateIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::AllocateIdsResponse>;
}

// sync client

pub struct DatastoreClient {
    async_client: DatastoreAsyncClient,
}

impl DatastoreClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        DatastoreAsyncClient::new(host, port, tls).map(|c| {
            DatastoreClient {
                async_client: c,
            }
        })
    }
}

impl Datastore for DatastoreClient {
    fn Lookup(&self, p: super::datastore::LookupRequest) -> ::grpc::result::GrpcResult<super::datastore::LookupResponse> {
        ::futures::Future::wait(self.async_client.Lookup(p))
    }

    fn RunQuery(&self, p: super::datastore::RunQueryRequest) -> ::grpc::result::GrpcResult<super::datastore::RunQueryResponse> {
        ::futures::Future::wait(self.async_client.RunQuery(p))
    }

    fn BeginTransaction(&self, p: super::datastore::BeginTransactionRequest) -> ::grpc::result::GrpcResult<super::datastore::BeginTransactionResponse> {
        ::futures::Future::wait(self.async_client.BeginTransaction(p))
    }

    fn Commit(&self, p: super::datastore::CommitRequest) -> ::grpc::result::GrpcResult<super::datastore::CommitResponse> {
        ::futures::Future::wait(self.async_client.Commit(p))
    }

    fn Rollback(&self, p: super::datastore::RollbackRequest) -> ::grpc::result::GrpcResult<super::datastore::RollbackResponse> {
        ::futures::Future::wait(self.async_client.Rollback(p))
    }

    fn AllocateIds(&self, p: super::datastore::AllocateIdsRequest) -> ::grpc::result::GrpcResult<super::datastore::AllocateIdsResponse> {
        ::futures::Future::wait(self.async_client.AllocateIds(p))
    }
}

// async client

pub struct DatastoreAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Lookup: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::datastore::LookupRequest, super::datastore::LookupResponse>>,
    method_RunQuery: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::datastore::RunQueryRequest, super::datastore::RunQueryResponse>>,
    method_BeginTransaction: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::datastore::BeginTransactionRequest, super::datastore::BeginTransactionResponse>>,
    method_Commit: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::datastore::CommitRequest, super::datastore::CommitResponse>>,
    method_Rollback: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::datastore::RollbackRequest, super::datastore::RollbackResponse>>,
    method_AllocateIds: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::datastore::AllocateIdsRequest, super::datastore::AllocateIdsResponse>>,
}

impl DatastoreAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            DatastoreAsyncClient {
                grpc_client: c,
                method_Lookup: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/google.datastore.v1beta3.Datastore/Lookup".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_RunQuery: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/google.datastore.v1beta3.Datastore/RunQuery".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_BeginTransaction: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/google.datastore.v1beta3.Datastore/BeginTransaction".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Commit: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/google.datastore.v1beta3.Datastore/Commit".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_Rollback: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/google.datastore.v1beta3.Datastore/Rollback".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
                method_AllocateIds: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/google.datastore.v1beta3.Datastore/AllocateIds".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl DatastoreAsync for DatastoreAsyncClient {
    fn Lookup(&self, p: super::datastore::LookupRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::LookupResponse> {
        self.grpc_client.call_unary(p, self.method_Lookup.clone())
    }

    fn RunQuery(&self, p: super::datastore::RunQueryRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::RunQueryResponse> {
        self.grpc_client.call_unary(p, self.method_RunQuery.clone())
    }

    fn BeginTransaction(&self, p: super::datastore::BeginTransactionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::BeginTransactionResponse> {
        self.grpc_client.call_unary(p, self.method_BeginTransaction.clone())
    }

    fn Commit(&self, p: super::datastore::CommitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::CommitResponse> {
        self.grpc_client.call_unary(p, self.method_Commit.clone())
    }

    fn Rollback(&self, p: super::datastore::RollbackRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::RollbackResponse> {
        self.grpc_client.call_unary(p, self.method_Rollback.clone())
    }

    fn AllocateIds(&self, p: super::datastore::AllocateIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::AllocateIdsResponse> {
        self.grpc_client.call_unary(p, self.method_AllocateIds.clone())
    }
}

// sync server

pub struct DatastoreServer {
    async_server: DatastoreAsyncServer,
}

struct DatastoreServerHandlerToAsync {
    handler: ::std::sync::Arc<Datastore + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl DatastoreAsync for DatastoreServerHandlerToAsync {
    fn Lookup(&self, p: super::datastore::LookupRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::LookupResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Lookup(p)
        })
    }

    fn RunQuery(&self, p: super::datastore::RunQueryRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::RunQueryResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.RunQuery(p)
        })
    }

    fn BeginTransaction(&self, p: super::datastore::BeginTransactionRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::BeginTransactionResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.BeginTransaction(p)
        })
    }

    fn Commit(&self, p: super::datastore::CommitRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::CommitResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Commit(p)
        })
    }

    fn Rollback(&self, p: super::datastore::RollbackRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::RollbackResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Rollback(p)
        })
    }

    fn AllocateIds(&self, p: super::datastore::AllocateIdsRequest) -> ::grpc::futures_grpc::GrpcFutureSend<super::datastore::AllocateIdsResponse> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.AllocateIds(p)
        })
    }
}

impl DatastoreServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Datastore + Send + Sync + 'static>(addr: A, h: H) -> Self {
        let h = DatastoreServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        DatastoreServer {
            async_server: DatastoreAsyncServer::new(addr, h),
        }
    }
}

// async server

pub struct DatastoreAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl DatastoreAsyncServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : DatastoreAsync + 'static + Sync + Send + 'static>(addr: A, h: H) -> Self {
        let service_definition = DatastoreAsyncServer::new_service_def(h);
        DatastoreAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(addr, service_definition),
        }
    }

    pub fn new_service_def<H : DatastoreAsync + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/google.datastore.v1beta3.Datastore/Lookup".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Lookup(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/google.datastore.v1beta3.Datastore/RunQuery".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.RunQuery(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/google.datastore.v1beta3.Datastore/BeginTransaction".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.BeginTransaction(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/google.datastore.v1beta3.Datastore/Commit".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Commit(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/google.datastore.v1beta3.Datastore/Rollback".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Rollback(p))
                    },
                ),
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/google.datastore.v1beta3.Datastore/AllocateIds".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.AllocateIds(p))
                    },
                ),
            ],
        )
    }
}
