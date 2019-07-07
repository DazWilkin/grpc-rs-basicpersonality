extern crate grpcio;
// extern crate basicpersonality_proto;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
use basicpersonality_proto::basicpersonality::ThingRequest;
use basicpersonality_proto::basicpersonality_grpc::BasicPersonalityClient;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(":50051");
    let client = BasicPersonalityClient::new(ch);

    let mut thing = Thing::new();
    thing.set_name("Thing");
    
    let mut rqst = ThingRequest::new();
    req.set_thing(thing);

    let resp = client.put(&rqst).expect("rpc");
    println!("Received: {}", resp.get_status());
}