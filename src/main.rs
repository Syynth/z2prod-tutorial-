use actix_web;
use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:0").unwrap())?.await
}
