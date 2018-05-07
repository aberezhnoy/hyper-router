extern crate hyper;

use std::collections::HashMap;
use hyper::Method;
use hyper::server::{Request, Response};

pub type MiddlewareCallback = fn(&Request, &mut Response, &mut HashMap<&str, usize>);

pub struct Middleware {
    pub steps: HashMap<Method, Vec<MiddlewareCallback>>
}

impl Middleware {
    pub fn new() -> Self {
        return Middleware {
            steps: HashMap::new()
        }
    }

    pub fn get(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Get, steps);

        return self;
    }

    pub fn get_(mut self, step: MiddlewareCallback) -> Self {
        self.steps.insert(Method::Get, vec![step]);

        return self;
    }

    pub fn post(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Post, steps);

        return self;
    }

    pub fn post_(mut self, step: MiddlewareCallback) -> Self {
        self.steps.insert(Method::Post, vec![step]);

        return self;
    }

    pub fn put(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Put, steps);

        return self;
    }

    pub fn delete(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Delete, steps);

        return self;
    }

    pub fn options(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Options, steps);

        return self;
    }

    pub fn head(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Head, steps);

        return self;
    }

    pub fn patch(mut self, steps: Vec<MiddlewareCallback>) -> Self {
        self.steps.insert(Method::Patch, steps);

        return self;
    }

    pub fn step_for(&self, method: &Method) -> Result<&Vec<MiddlewareCallback>, ()> {
        return self.steps
            .get(method)
            .ok_or(());
    }
}
