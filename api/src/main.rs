extern crate actix_web;
extern crate listenfd;
extern crate rust_todo;

use actix_web::{server, App, HttpRequest, Responder};
use listenfd::ListenFd;
use rust_todo::*;

fn index(_req: &HttpRequest) -> impl Responder {
    let connection = establish_connection();
    let todos = get_todos(&connection);
    todos
        .iter()
        .map(|todo| {
            format!(
                "{title}: {body}",
                title = todo.title.clone(),
                body = todo.body.clone()
            )
        })
        .collect::<Vec<String>>()
        .join("|")
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| App::new().resource("/", |r| r.f(index)));

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
