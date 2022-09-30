use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration;
use zero2prod::email_client::EmailClient;
use zero2prod::startup;
use zero2prod::telemetry;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let server = startup::build(configuration).await?;
    server.await?;
    Ok(())
}
