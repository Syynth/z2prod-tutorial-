use std::net::TcpListener;

use actix_web;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let cfg = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", cfg.application_port);
    let connection_pool = PgPool::connect(&cfg.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
