extern crate actix_web;
extern crate listenfd;

use listenfd::ListenFd;
use actix_web::{server, App, HttpRequest, Responder};

fn index(_req: &HttpRequest) -> impl Responder {
    "Fast start"
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        println!("listen?");
        server.listen(l)
    } else {
        println!("bind?");
        server.bind("0.0.0.0:3000").unwrap()
    };
    println!("Running on 0.0.0.0:3000");
    server.run();
}