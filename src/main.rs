extern crate hyper;
extern crate futures;

mod router;
use router::router::Router;
use router::middleware::Middleware;

use hyper::server::{Request, Response};
use hyper::Method::{Get, Post};
use hyper::Uri;

use std::collections::HashMap;
use std::any::Any;

/*use hyper::HttpVersion;
use hyper::Headers;
use hyper::server::Http;
use hyper::Server;*/

struct MyObj {
    my_str: String
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
        .post_(|req, res, ctx| {
            res.headers_mut().set_raw("h1", "v1");
            println!("Hey3 {}", req.path());
        })
    );

    let resp = router
        .invoke(&mut Request::new(Post, "/p2".parse::<Uri>().unwrap()))
        .expect("error");

    println!("{:#?}", resp);*/

    let mut m: HashMap<&str, Box<Any>> = HashMap::new();

    m.insert("x", Box::new(String::from("my val")));

    /*let s = String::from("my str");
    let y = 55;*/
    /*m.insert("str", s);*/

    //let x = test(&y);

    //let r = test();

    //println!("{:?}", r.downcast_ref::<String>());

    println!("{:?}", m.get("x").unwrap().downcast_ref::<String>());
}


fn test() -> Box<Any> {
    /*if x.downcast_ref::<String>().is_some() {
        println!("its string");
    }

    return 1*/
    let s = String::from("sdfsfdsdf");

    return Box::new(s);
}