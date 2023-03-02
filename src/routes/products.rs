use crate::error::APIError;
use crate::repository::get_product;
use crate::repository::get_products;
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use firestore::*;
use serde_json::{json, to_string};
use uuid::Uuid;

fn get_token<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers()
        .get("x-apigateway-api-userinfo")?
        .to_str()
        .ok()
}

pub async fn products(
    db: web::Data<FirestoreDb>,
    req: HttpRequest,
) -> Result<HttpResponse, APIError> {
    if let Some(token) = get_token(&req) {
        tracing::info!(token, "x-apigateway-api-userinfo Firebase Token Found");
    }

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
