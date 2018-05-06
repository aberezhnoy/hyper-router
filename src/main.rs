extern crate hyper;
extern crate futures;

mod router;
use router::router::Router;
use router::middleware::Middleware;

use hyper::server::{Request, Response};
use hyper::Method::{Get, Post};
use hyper::Uri;

use std::collections::HashMap;

/*use hyper::HttpVersion;
use hyper::Headers;
use hyper::server::Http;
use hyper::Server;*/



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
        .post(vec![
            |req: &Request, _resp: &mut Response, _ctx: &mut HashMap<&str, usize>| {
                _resp.headers_mut().set_raw("h1", "v1");

                let x = *String::from("sfd");

                //_ctx.insert("key1", x);

                println!("Hey2 {}", req.path());
            }
        ])
    );

    let resp = router
        .invoke(&mut Request::new(Post, "/p2".parse::<Uri>().unwrap()))
        .expect("error");

    println!("{:#?}", resp);*/
}

fn test() {
    //let map: HashMap<&str, > = HashMap::new();
}