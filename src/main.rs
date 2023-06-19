use robust_rust::{
    configuration::get_configuration, startup::Application, telemetry::get_subscriber,
    telemetry::init_subscriber,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("robust_rust".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let server = Application::build(configuration).await?;
    server.run_until_stopped().await?;
    Ok(())
}
