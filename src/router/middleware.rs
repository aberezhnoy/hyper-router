extern crate hyper;

use std::collections::HashMap;
use hyper::Method;
use hyper::server::Request;

pub type MiddlewareCallback = fn(&Request, &(Fn() -> ()));

pub struct Middleware {
    pub steps: HashMap<Method, Vec<MiddlewareCallback>>
}

impl Middleware {
    pub fn new() -> Middleware {
        return Middleware {
            steps: HashMap::new()
        }
    }

    pub fn get(&mut self, steps: Vec<MiddlewareCallback>) -> &Middleware {
        self.steps.insert(Method::Get, steps);

        return self;
    }

    pub fn post(&mut self, steps: Vec<MiddlewareCallback>) -> &Middleware {
        self.steps.insert(Method::Post, steps);

        return self;
    }

    pub fn step_for(&self, method: &Method) -> Result<&Vec<MiddlewareCallback>, ()> {
        return self.steps.get(method).ok_or(());
    }
}
