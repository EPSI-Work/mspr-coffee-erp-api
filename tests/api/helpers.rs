use erp_api::configuration::get_configuration;
use erp_api::startup::Application;
use erp_api::telemetry::{get_subscriber, init_subscriber};
use firestore::FirestoreDb;
use once_cell::sync::Lazy;
use std::env::set_var;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    // We cannot assign the output of `get_subscriber` to a variable based on the value of
    //`TEST_LOG` because the sink is part of the type returned by `get_subscriber`,
    //therefore they are not the same type.
    // We could work around it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db: FirestoreDb,
}

// The function is asynchronous now!
pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Create and migrate the database

    //configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());

    // Setup Firestore
    set_var(
        "GOOGLE_APPLICATION_CREDENTIALS",
        configuration.firebase.credential,
    );
    let firestore_database = FirestoreDb::new(configuration.firebase.project_id)
        .await
        .expect("Failed to setup firebase connection for testing");

    // spawn app in the background
    let _ = tokio::spawn(application.run_until_stopped());
    TestApp {
        address,
        port: application_port,
        db: firestore_database,
    }
}
