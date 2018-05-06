extern crate futures;
extern crate hyper;

use std::collections::HashMap;
use hyper::server::{Request, Response, Service};
use futures::future::Future;

use router::middleware::{Middleware, MiddlewareCallback};

pub struct Router {
    routes: HashMap<String, Middleware>
}

impl Router {
    pub fn new() -> Self {
        return Router {
            routes: HashMap::new()
        };
    }

    pub fn add(&mut self, path: &str, middleware: Middleware) -> &Self {
        self.routes.insert(String::from(path), middleware);

        return self;
    }

    pub fn invoke(&self, request: &Request) -> Result<Response, ()> {
        let m = self.routes.get(request.path());

        if m.is_none() {
            return Err(());
        }

        let x = m.unwrap();

        let steps = x.step_for(request.method());

        if steps.is_err() {
            return Err(());
        }

        let _steps = steps.unwrap();

        let mut response = Response::new();
        self.do_call(_steps, &request, &mut response,0);

        return Ok(response);
    }

    fn do_call(&self, steps: &Vec<MiddlewareCallback>, request: &Request, response: &mut Response, _index: usize) {
        let mut ctx: HashMap<&str, usize> = HashMap::new();

        for step in steps {
            step(request, response, &mut ctx);
        }
    }
}

impl Service for Router {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let resp = self.invoke(&req).unwrap();

        return Box::new(futures::future::ok(resp));
    }
}