extern crate futures;
extern crate grpc;
extern crate Talk_microservice;
extern crate httpbis;
extern crate tls_api;
extern crate tls_api_native_tls;
extern crate env_logger;

use Talk_microservice::Talk::*;
use Talk_microservice::Talk_grpc::*;
use grpc::ClientStubExt;

fn main() {
    env_logger::init();

    let name = "Paul".to_string();

    let port = 50051;

    let client_conf = Default::default();

    let client = GreeterClient::new_plain("127.0.0.1", port, client_conf).unwrap();

    let mut req = HelloRequest::new();
    req.set_name(name);

    let resp = client.say_hello(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}
