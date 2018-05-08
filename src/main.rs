extern crate hyper;
extern crate futures;

mod router;
use router::router::Router;
use router::middleware::Middleware;
use router::utils::InsertBoxed;
use router::utils::get_downcast_ref;

use hyper::server::{Request, Response};
use hyper::Method::{Get, Post};
use hyper::Uri;

use std::collections::HashMap;
use std::any::Any;

/*use hyper::HttpVersion;
use hyper::Headers;
use hyper::server::Http;
use hyper::Server;*/

#[derive(Debug)]
struct MyObj {
    pub my_str: String
}

#[derive(Debug)]
struct MyCtx {
    pub v1: String,
    pub v2: u8,
    pub v3: MyObj
}

trait MyNew {
    fn new() -> Self;
}

impl MyNew for MyCtx {
    fn new() -> Self {
        MyCtx {
            v1: String::from("sdfsdf"),
            v2: 0,
            v3: MyObj { my_str: String::from("sdfsdf") }
        }
    }
}

fn main() {
    /*let addr = "127.0.0.1:3000"
        .parse()
        .unwrap();

    let server = Http::new()
        .bind(&addr, || Ok(EchoService))
        .unwrap();

    server
        .run()
        .unwrap();*/

    /*let mut router = Router::new();

    router.add("/p2", Middleware::new()
        .post::<MyCtx>(vec![
            |req, res, ctx| {
                ctx.insert("k1", Box::new("test_string"));
                ctx.insert("k2", Box::new(MyObj {
                    my_str: String::from("test_string2"),
                }));
            },

            |req, res, ctx| {
                let x: &str = ctx.get("k1").unwrap().downcast_ref::<&str>().unwrap();
                let y: &MyObj = ctx.get("k2").unwrap().downcast_ref::<MyObj>().unwrap();

                res.headers_mut().set_raw("h1", x);
                res.headers_mut().set_raw("h2", y.my_str.as_str());
            }
        ])
    );

    let resp = router
        .invoke(&mut Request::new(Post, "/p2".parse::<Uri>().unwrap()))
        .expect("error");

    println!("{:#?}", resp);*/

    test::<MyCtx>(vec![
        |x, y| {
            println!("{:?}", y);
        }
    ]);

}

fn test<T: MyNew>(cb: Vec<fn(u8, &T)>) {
    let y = T::new();

    for x in cb {
        x(1, &y)
    }
}