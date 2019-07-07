// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_BASIC_PERSONALITY_PUT_THING: ::grpcio::Method<super::basicpersonality::ThingRequest, super::basicpersonality::ThingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/basicpersonality.BasicPersonality/PutThing",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BASIC_PERSONALITY_GET_THING: ::grpcio::Method<super::basicpersonality::ThingRequest, super::basicpersonality::ThingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/basicpersonality.BasicPersonality/GetThing",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BASIC_PERSONALITY_WAIT_THING: ::grpcio::Method<super::basicpersonality::ThingRequest, super::basicpersonality::ThingResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/basicpersonality.BasicPersonality/WaitThing",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct BasicPersonalityClient {
    client: ::grpcio::Client,
}

impl BasicPersonalityClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BasicPersonalityClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn put_thing_opt(&self, req: &super::basicpersonality::ThingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::basicpersonality::ThingResponse> {
        self.client.unary_call(&METHOD_BASIC_PERSONALITY_PUT_THING, req, opt)
    }

    pub fn put_thing(&self, req: &super::basicpersonality::ThingRequest) -> ::grpcio::Result<super::basicpersonality::ThingResponse> {
        self.put_thing_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_thing_async_opt(&self, req: &super::basicpersonality::ThingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::basicpersonality::ThingResponse>> {
        self.client.unary_call_async(&METHOD_BASIC_PERSONALITY_PUT_THING, req, opt)
    }

    pub fn put_thing_async(&self, req: &super::basicpersonality::ThingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::basicpersonality::ThingResponse>> {
        self.put_thing_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_thing_opt(&self, req: &super::basicpersonality::ThingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::basicpersonality::ThingResponse> {
        self.client.unary_call(&METHOD_BASIC_PERSONALITY_GET_THING, req, opt)
    }

    pub fn get_thing(&self, req: &super::basicpersonality::ThingRequest) -> ::grpcio::Result<super::basicpersonality::ThingResponse> {
        self.get_thing_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_thing_async_opt(&self, req: &super::basicpersonality::ThingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::basicpersonality::ThingResponse>> {
        self.client.unary_call_async(&METHOD_BASIC_PERSONALITY_GET_THING, req, opt)
    }

    pub fn get_thing_async(&self, req: &super::basicpersonality::ThingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::basicpersonality::ThingResponse>> {
        self.get_thing_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_thing_opt(&self, req: &super::basicpersonality::ThingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::basicpersonality::ThingResponse> {
        self.client.unary_call(&METHOD_BASIC_PERSONALITY_WAIT_THING, req, opt)
    }

    pub fn wait_thing(&self, req: &super::basicpersonality::ThingRequest) -> ::grpcio::Result<super::basicpersonality::ThingResponse> {
        self.wait_thing_opt(req, ::grpcio::CallOption::default())
    }

    pub fn wait_thing_async_opt(&self, req: &super::basicpersonality::ThingRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::basicpersonality::ThingResponse>> {
        self.client.unary_call_async(&METHOD_BASIC_PERSONALITY_WAIT_THING, req, opt)
    }

    pub fn wait_thing_async(&self, req: &super::basicpersonality::ThingRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::basicpersonality::ThingResponse>> {
        self.wait_thing_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait BasicPersonality {
    fn put_thing(&mut self, ctx: ::grpcio::RpcContext, req: super::basicpersonality::ThingRequest, sink: ::grpcio::UnarySink<super::basicpersonality::ThingResponse>);
    fn get_thing(&mut self, ctx: ::grpcio::RpcContext, req: super::basicpersonality::ThingRequest, sink: ::grpcio::UnarySink<super::basicpersonality::ThingResponse>);
    fn wait_thing(&mut self, ctx: ::grpcio::RpcContext, req: super::basicpersonality::ThingRequest, sink: ::grpcio::UnarySink<super::basicpersonality::ThingResponse>);
}

pub fn create_basic_personality<S: BasicPersonality + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BASIC_PERSONALITY_PUT_THING, move |ctx, req, resp| {
        instance.put_thing(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BASIC_PERSONALITY_GET_THING, move |ctx, req, resp| {
        instance.get_thing(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BASIC_PERSONALITY_WAIT_THING, move |ctx, req, resp| {
        instance.wait_thing(ctx, req, resp)
    });
    builder.build()
}
