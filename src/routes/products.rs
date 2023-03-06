use crate::entity::{generate_pagination_response, Product};
use crate::error::APIError;
use crate::repository::get_product;
use crate::repository::get_products;
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use firestore::*;
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::TokenData;
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
}

fn get_token<'a>(req: &'a HttpRequest) -> Option<TokenData<Claims>> {
    let token = req
        .headers()
        .get("x-apigateway-api-userinfo")
        .map(|value| value.to_str().ok())
        .unwrap_or_else(|| None);

    if let Some(token) = token {
        let token_decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("".as_ref()),
            &Validation::default(),
        );

        match token_decoded {
            Ok(token) => Some(token),
            Err(err) => {
                dbg!(err);
                None
            }
        }

        // let bytes = decode(token).unwrap();
        // let utf8_string = String::from_utf;
        //Some(())
    } else {
        None
    }
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page: u64,
    pub size: u64,
}

// #[tracing::instrument(
//     name = "Get list of products",
//     skip(db, req),
//     fields(
//         subscriber_email = %form.email,
//         subscriber_name = %form.name
//     )
// )]
pub async fn products(
    db: web::Data<FirestoreDb>,
    //req: HttpRequest,
    pagination: web::Query<Pagination>,
) -> Result<HttpResponse, APIError> {
    // if let Some(token) = get_token(&req) {
    //     //tracing::info!(token, "x-apigateway-api-userinfo Firebase Token Found");
    // } else {
    //     dbg!("no token found");
    // }

    let products = get_products(&db)
        .await
        .context("Failed to get products from database.")?;

    let total_results = products.len().try_into().unwrap();
    let response = generate_pagination_response::<Product>(
        products,
        total_results,
        pagination.page,
        pagination.size,
        "products",
    );

    let json = to_string(&response).unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(json))
}

pub async fn product(
    db: web::Data<FirestoreDb>,
    id: web::Path<Uuid>,
    req: HttpRequest,
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
