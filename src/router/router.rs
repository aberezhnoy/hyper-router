extern crate hyper;

use std::collections::HashMap;
use hyper::Method;
use hyper::server::Request;

use router::middleware::{Middleware, MiddlewareCallback};

pub struct Router {
    routes: HashMap<String, Middleware>
}

impl Router {
    pub fn new() -> Router {
        return Router {
            routes: HashMap::new()
        };
    }

    pub fn add(&mut self, path: &str, middleware: Middleware) -> &Router {
        self.routes.insert(String::from(path), middleware);

        return self;
    }

    pub fn invoke(&self, request: &Request) -> Result<(), ()> {
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

        self.do_call(_steps, request, 0);

        return Ok(());
    }

    pub fn do_call(&self, steps: &Vec<MiddlewareCallback>, request: &Request, index: usize) {
        let m: Option<&MiddlewareCallback> = steps.get(index);

        if m.is_none() {
            return;
        }

        let x = m.unwrap();

        x(request, &|| {
            self.do_call(steps, request, index + 1);
        });
    }
}
