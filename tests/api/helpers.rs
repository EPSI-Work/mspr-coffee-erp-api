use erp_api::configuration::get_configuration;
use erp_api::startup::Application;
use erp_api::telemetry::{get_subscriber_without_elk, init_subscriber};
use firestore::FirestoreDb;
use once_cell::sync::Lazy;
use tracing::Level;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = Level::INFO;
    let subscriber_name = "test".to_string();

    // We cannot assign the output of `get_subscriber` to a variable based on the value of
    //`TEST_LOG` because the sink is part of the type returned by `get_subscriber`,
    //therefore they are not the same type.
    // We could work around it, but this is the most straight-forward way of moving forward.
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            get_subscriber_without_elk(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber =
            get_subscriber_without_elk(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db: FirestoreDb,
}

impl TestApp {
    pub async fn get_product(&self, id: String) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!("{}/products/{}", &self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_products(&self, page: u64, size: u64) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!(
                "{}/products?page={}&size={}",
                &self.address, page, size
            ))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

// The function is asynchronous now!
pub async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a random OS port
        c.application.port = 0;
        c
    };

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let address = format!("http://127.0.0.1:{}", application.port());
    let firestore_database = application.db();

    // spawn app in the background
    let _ = tokio::spawn(application.run_until_stopped());
    TestApp {
        address,
        port: application_port,
        db: firestore_database,
    }
}
