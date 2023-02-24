use clap::{command, Parser};
use erp_api::entity::Product;
use firestore::FirestoreDb;
use std::env::set_var;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of the file to import
    #[arg(short, long)]
    file_path: String,

    /// Id of the firebase projet
    #[arg(long)]
    firebase_id: String,

    /// Service Account token for the firebase projet
    #[arg(long)]
    firebase_token: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(args.file_path).expect("File not found");

    set_var("GOOGLE_APPLICATION_CREDENTIALS", args.firebase_token);

    let firestore_database = FirestoreDb::new(&args.firebase_id)
        .await
        .expect("Failed to setup firebase connection");

    let products: Vec<Product> =
        serde_json::from_str(&contents).expect("Failed to deserialize json file");

    for product in products {
        let _: Product = firestore_database
            .fluent()
            .insert()
            .into("products")
            .document_id(&product.id.to_string())
            .object(&product)
            .execute()
            .await
            .expect("Failed to insert object in firestore");
    }
}
