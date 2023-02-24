use crate::error::APIError;
use crate::repository::get_product;
use crate::repository::get_products;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use firestore::*;
use serde_json::{json, to_string};
use uuid::Uuid;

pub async fn products(db: web::Data<FirestoreDb>) -> Result<HttpResponse, APIError> {
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
) -> Result<HttpResponse, APIError> {
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
