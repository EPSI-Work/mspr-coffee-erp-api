use erp_api::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber_without_elk, init_subscriber},
};
use tracing::Level;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let subscriber = get_subscriber_without_elk("zero2prod".into(), Level::INFO, std::io::stdout);
    init_subscriber(subscriber);

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
