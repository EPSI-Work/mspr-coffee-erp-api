use crate::entity::{Product, Reseller};
use firestore::errors::FirestoreError;
use firestore::FirestoreDb;
use firestore::*;
use futures::stream::BoxStream;
use futures::StreamExt;
use uuid::Uuid;

const COLLECTION_NAME: &str = "products";

pub async fn get_products(
    db: &FirestoreDb,
    reseller: &Reseller,
) -> Result<Vec<Product>, FirestoreError> {
    let parent_path = db.parent_path("resellers", reseller.id.to_string())?;

    // add limit and offset
    let products_return: BoxStream<Product> = db
        .fluent()
        .select()
        .fields(paths!(Product::{id, name, stock, created_at}))
        .from(COLLECTION_NAME)
        .parent(parent_path)
        .obj()
        .stream_query()
        .await?;

    let as_vec: Vec<Product> = products_return.collect().await;
    Ok(as_vec)
}

pub async fn get_product(
    db: &FirestoreDb,
    id: Uuid,
    reseller: &Reseller,
) -> Result<Option<Product>, FirestoreError> {
    let parent_path = db.parent_path("resellers", reseller.id.to_string())?;

    let product_return: Option<Product> = db
        .fluent()
        .select()
        .by_id_in(COLLECTION_NAME)
        .parent(parent_path)
        .obj()
        .one(&id.to_string())
        .await?;

    Ok(product_return)
}
