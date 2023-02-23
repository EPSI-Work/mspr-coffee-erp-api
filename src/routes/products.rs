use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::ResponseError;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use chrono::{DateTime, Utc};
use firestore::errors::FirestoreError;
use firestore::*;
use futures::stream::BoxStream;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, to_string};
use uuid::Uuid;

const COLLECTION_NAME: &'static str = "products";

#[derive(thiserror::Error)]
pub enum ProductError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for ProductError {
    fn status_code(&self) -> StatusCode {
        match self {
            ProductError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Debug for ProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub details: Option<Detail>,
    pub stock: i64,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Utc>,
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.details == other.details
            && self.stock == other.stock
            && self.created_at == other.created_at
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Detail {
    pub price: f64,
    pub description: String,
    pub color: String,
}

impl PartialEq for Detail {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
            && self.description == other.description
            && self.color == other.color
    }
}

pub async fn products(db: web::Data<FirestoreDb>) -> Result<HttpResponse, ProductError> {
    let products = get_products(&db)
        .await
        .context("Failed to get products from database.")?;

    let json = to_string(&products).unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json))
}

pub async fn product(
    db: web::Data<FirestoreDb>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ProductError> {
    let product = get_product(&db, *id)
        .await
        .context("Failed to get product from database.")?;

    match product {
        None => Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body(json!({"message": "products not found"}).to_string())),
        Some(product) => {
            let json = to_string(&product).unwrap();

            Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json))
        }
    }
}

async fn get_products(db: &FirestoreDb) -> Result<Vec<Product>, FirestoreError> {
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

async fn get_product(db: &FirestoreDb, id: Uuid) -> Result<Option<Product>, FirestoreError> {
    let product_return: Option<Product> = db
        .fluent()
        .select()
        .by_id_in(COLLECTION_NAME)
        .obj()
        .one(&id.to_string())
        .await?;

    Ok(product_return)
}
