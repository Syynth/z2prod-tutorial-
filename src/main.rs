use sqlx::PgPool;
use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(get_subscriber("zero2prod".into(), "info".into()));

    let cfg = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", cfg.application_port);
    let connection_pool = PgPool::connect(&cfg.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
