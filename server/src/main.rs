#![allow(dead_code, unused_imports, unused_variables)]

extern crate actix_rt;
extern crate actix_web;
extern crate env_logger;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};
use actix_files::{Files};

fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("example-actix-yew");

    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/", "./static").index_file("index.html")))
        .bind("127.0.0.1:8080")?
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}
