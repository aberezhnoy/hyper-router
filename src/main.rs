extern crate hyper;
extern crate futures;

mod echoservice;

mod router;
use router::router::Router;
use router::middleware::Middleware;

use hyper::server::Request;

use hyper::Method::{Get};
use hyper::Uri;
use hyper::HttpVersion;
use hyper::Headers;

use hyper::server::Http;
use echoservice::EchoService;
use hyper::Server;

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


    let mut m1 = Middleware::new();
    m1.get(vec![
        |r: &Request, next| {
            println!("s1 {}", r.path());
            next();
        },

        |r: &Request, next| {
            println!("s2 {}", r.path());
            next();
        }
    ]);

    let mut r = Router::new();
    r.add("/p1", m1);

    let req: Request = Request::new(Get, "/p1".parse::<Uri>().unwrap());

    r.invoke(&req);
}
