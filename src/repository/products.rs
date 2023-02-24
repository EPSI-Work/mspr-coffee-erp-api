use crate::entity::Product;
use firestore::errors::FirestoreError;
use firestore::FirestoreDb;
use firestore::*;
use futures::stream::BoxStream;
use futures::StreamExt;
use uuid::Uuid;

const COLLECTION_NAME: &'static str = "products";

pub async fn get_products(db: &FirestoreDb) -> Result<Vec<Product>, FirestoreError> {
    let products_return: BoxStream<Product> = db
        .fluent()
        .select()
        .fields(paths!(Product::{id, name, stock, created_at}))
        .from(COLLECTION_NAME)
        .obj()
        .stream_query()
        .await?;

    let as_vec: Vec<Product> = products_return.collect().await;
    Ok(as_vec)
}

pub async fn get_product(db: &FirestoreDb, id: Uuid) -> Result<Option<Product>, FirestoreError> {
    let product_return: Option<Product> = db
        .fluent()
        .select()
        .by_id_in(COLLECTION_NAME)
        .obj()
        .one(&id.to_string())
        .await?;

    Ok(product_return)
}
