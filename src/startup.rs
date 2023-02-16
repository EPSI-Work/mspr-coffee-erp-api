use crate::{
    configuration::Settings,
    routes::{health_check, product, products},
};
use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use firestore::*;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let firestore_database = FirestoreDb::new(&configuration.application.firebase_project_id)
            .await
            .expect("Failed to setup firebase connection");

        let server = run(listener, firestore_database)?;
        // We "save" the bound port in one of `Application`'s fields
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

#[derive(Debug)]
pub struct ApplicationBaseUrl(pub String);

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listener: TcpListener, firestore_db: FirestoreDb) -> Result<Server, std::io::Error> {
    let firestore_connection = Data::new(firestore_db);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/products/{id}", web::get().to(product))
            .route("/products", web::get().to(products))
            .route("/health_check", web::get().to(health_check))
            .app_data(firestore_connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
