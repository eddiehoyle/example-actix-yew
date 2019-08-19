#![allow(dead_code, unused_imports, unused_variables)]

extern crate actix_web;
extern crate actix_rt;
extern crate env_logger;

use std::{env, io};
use actix_files as fs;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    web, middleware,
    App, Result, HttpResponse, HttpRequest, HttpServer};

fn main() -> io::Result<()> {

    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let sys = actix_rt::System::new("example-actix-yew");

    HttpServer::new(move ||
        App::new()
            .wrap(middleware::Logger::default())
//            .service(web::resource("/")
//                .route(web::get().to(|_: HttpRequest| HttpResponse::Ok())))
//            .service(fs::Files::new("/foo", "./static").index_file("index.html"))
            .service(fs::Files::new("/", "./static").index_file("index.html"))

    )
        .bind("127.0.0.1:8080")?
        .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}