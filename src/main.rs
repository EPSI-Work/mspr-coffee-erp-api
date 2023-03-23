use erp_api::{
    configuration::get_configuration,
    observability::{get_subscriber, init_subscriber},
    startup::Application,
};
use tracing::Level;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let subscriber = get_subscriber("zero2prod".into(), Level::INFO, std::io::stdout);
    init_subscriber(subscriber);

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
