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
use secrecy::ExposeSecret;
use std::env::set_var;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// A new type to hold the newly built server and its port
pub struct Application {
    port: u16,
    server: Server,
    db: FirestoreDb,
}

#[derive(Debug, Clone)]
pub struct CloudFunction {
    pub host: String,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        // Setup Firestore
        set_var(
            "GOOGLE_APPLICATION_CREDENTIALS",
            configuration.firebase.credential.expose_secret(),
        );

        //set_var("FIRESTORE_EMULATOR_HOST", configuration.firebase.host);

        let db = FirestoreDb::new(configuration.firebase.project_id.expose_secret())
            .await
            .expect("Failed to setup firebase connection");

        let cloud_function = CloudFunction {
            host: configuration.cloudfunction.host.expose_secret().clone(),
        };

        let server = run(listener, db.clone(), cloud_function)?;

        // We "save" the bound port in one of `Application`'s fields
        Ok(Self { port, server, db })
    }

    pub fn db(&self) -> FirestoreDb {
        self.db.clone()
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

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(
    listener: TcpListener,
    firestore_db: FirestoreDb,
    cloud_function_host: CloudFunction,
) -> Result<Server, std::io::Error> {
    let firestore_connection = Data::new(firestore_db);
    let cloud_function = Data::new(cloud_function_host);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/products/{id}", web::get().to(product))
            .route("/products", web::get().to(products))
            .route("/health_check", web::get().to(health_check))
            .app_data(firestore_connection.clone())
            .app_data(cloud_function.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
