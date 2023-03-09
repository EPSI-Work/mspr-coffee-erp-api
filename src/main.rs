use erp_api::{
    configuration::get_configuration,
    startup::Application,
    telemetry::{get_subscriber_without_elk, init_subscriber},
};
use tracing::Level;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    // let file_appender = tracing_appender::rolling::hourly("logs", "log");
    // let (file_sink, _guard) = tracing_appender::non_blocking(file_appender);

    // let subscriber = get_subscriber_with_elk(
    //     "zero2prod".into(),
    //     Level::INFO.into(),
    //     std::io::stdout,
    //     (
    //         configuration.logstach.host.expose_secret().to_string(),
    //         configuration.logstach.port,
    //     ),
    // );

    // let tracer = stdout::new_pipeline().install_simple();
    // // Create a tracing layer with the configured tracer
    // let telemetry = tracing_opentelemetry::layer::<_>().with_tracer(tracer);

    let subscriber = get_subscriber_without_elk("zero2prod".into(), Level::INFO, std::io::stdout);

    init_subscriber(subscriber);

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
