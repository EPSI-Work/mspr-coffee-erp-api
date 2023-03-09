use erp_api::configuration::get_configuration;
use erp_api::entity::{Product, Reseller, User};
use erp_api::routes::VerifyFirebaseToken;
use erp_api::startup::Application;
use erp_api::telemetry::{get_subscriber_without_elk, init_subscriber};
use fake::Fake;
use fake::Faker;
use firestore::FirestoreDb;
use once_cell::sync::Lazy;
use tracing::Level;
use wiremock::MockServer;

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
    pub cloud_function_server: MockServer,
}

const COLLECTION_PRODUCTS: &str = "products";
const COLLECTION_RESELLERS: &str = "resellers";
const COLLECTION_USERS: &str = "users";

impl TestApp {
    pub async fn setup_database(&self, product_number: u8) -> (Reseller, User, Vec<Product>) {
        let reseller: Reseller = Faker.fake();
        let mut products: Vec<Product> = Vec::new();
        let mut user: User = Faker.fake();

        // Creating a parent doc
        let reseller_firestore: Reseller = self
            .db
            .fluent()
            .insert()
            .into(COLLECTION_RESELLERS)
            .document_id(&reseller.id.to_string())
            .object(&reseller)
            .execute()
            .await
            .expect("Failed to insert reseller in test firestore database");

        // The doc path where we store our children
        let parent_path = self
            .db
            .parent_path(COLLECTION_RESELLERS, reseller.id.to_string())
            .expect("Failed to get parent path");

        for _ in 0..product_number {
            let product: Product = Faker.fake();
            // Create a child doc
            let product_firestore: Product = self
                .db
                .fluent()
                .insert()
                .into(COLLECTION_PRODUCTS)
                .document_id(&product.id.to_string())
                .parent(&parent_path)
                .object(&product)
                .execute()
                .await
                .expect("Failed to insert product in test firestore database");
            products.push(product_firestore);
        }

        user.reseller_id = reseller_firestore.id;
        // Create a child doc
        let user_firestore: User = self
            .db
            .fluent()
            .insert()
            .into(COLLECTION_USERS)
            .document_id(&user.firebase_id.to_string())
            .parent(&parent_path)
            .object(&user)
            .execute()
            .await
            .expect("Failed to insert user in test firestore database");

        (reseller_firestore, user_firestore, products)
    }

    pub async fn get_product(
        &self,
        id: String,
        api_key: String,
        firebase_token: VerifyFirebaseToken,
    ) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!(
                "{}/products/{}?api_key={}",
                &self.address, id, api_key
            ))
            .header("x-apigateway-api-userinfo", firebase_token.firebase_token)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_products(
        &self,
        page: u64,
        size: u64,
        api_key: String,
        firebase_token: VerifyFirebaseToken,
    ) -> reqwest::Response {
        reqwest::Client::new()
            .get(&format!(
                "{}/products?page={}&size={}&api_key={}",
                &self.address, page, size, api_key
            ))
            .header("x-apigateway-api-userinfo", firebase_token.firebase_token)
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

    // Launch a mock server to stand in for Postmark's API
    let cloud_function_server = MockServer::start().await;

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a random OS port
        c.application.port = 0;
        c.cloudfunction.host = cloud_function_server.uri().into();
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
        cloud_function_server,
    }
}
