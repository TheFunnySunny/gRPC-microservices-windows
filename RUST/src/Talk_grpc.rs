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


// server interface

pub trait Greeter {
    fn say_hello(&self, o: ::grpc::ServerHandlerContext, req: ::grpc::ServerRequestSingle<super::Talk::HelloRequest>, resp: ::grpc::ServerResponseUnarySink<super::Talk::HelloReply>) -> ::grpc::Result<()>;
}

// client

pub struct GreeterClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
}

impl ::grpc::ClientStub for GreeterClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        GreeterClient {
            grpc_client: grpc_client,
        }
    }
}

impl GreeterClient {
    pub fn say_hello(&self, o: ::grpc::RequestOptions, req: super::Talk::HelloRequest) -> ::grpc::SingleResponse<super::Talk::HelloReply> {
        let descriptor = ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
            name: ::grpc::rt::StringOrStatic::Static("/Talk.Greeter/SayHello"),
            streaming: ::grpc::rt::GrpcStreaming::Unary,
            req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
            resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
        });
        self.grpc_client.call_unary(o, req, descriptor)
    }
}

// server

pub struct GreeterServer;


impl GreeterServer {
    pub fn new_service_def<H : Greeter + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/Talk.Greeter",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::grpc::rt::ArcOrStatic::Static(&::grpc::rt::MethodDescriptor {
                        name: ::grpc::rt::StringOrStatic::Static("/Talk.Greeter/SayHello"),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: ::grpc::rt::ArcOrStatic::Static(&::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |ctx, req, resp| (*handler_copy).say_hello(ctx, req, resp))
                    },
                ),
            ],
        )
    }
}
