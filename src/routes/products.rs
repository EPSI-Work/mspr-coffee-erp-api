use crate::entity::{generate_pagination_response, FirebaseUser, Product, Reseller};
use crate::error::APIError;
use crate::repository::get_products;
use crate::repository::get_reseller;
use crate::repository::{get_product, get_user};
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use base64::decode;
use firestore::*;
use serde::Deserialize;
use serde_json::{json, to_string};
use tracing_actix_web::RequestId;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}

#[derive(Deserialize, Debug)]
pub struct APIKey {
    pub api_key: String,
}

fn get_firebase_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get("x-apigateway-api-userinfo")
        .map(|value| value.to_str().ok())
        .unwrap_or_else(|| None)
}

async fn check_authorization(
    api_key: String,
    token: String,
    db: &FirestoreDb,
) -> Result<Reseller, anyhow::Error> {
    let decoded_bytes = decode(&token).context("Impossible de décoder la chaîne Base64")?; // Décodage de la chaîne Base64

    let decoded_string =
        String::from_utf8(decoded_bytes).context("Les octets décodés ne sont pas valides UTF-8")?;

    let firebase_user =
        serde_json::from_str::<FirebaseUser>(&decoded_string).context("Failed to parse json")?;

    // check if the reseller with the given api key exist
    let reseller = get_reseller(&db, &api_key)
        .await
        .context("Failed to get products from database.")?;

    let reseller = reseller.context("No reseller found")?;

    // check if the user has a reseller
    let user = get_user(&db, &firebase_user.user_id, &reseller)
        .await
        .context("Failed to get products from database.")?;

    let user = user.context("No user found")?;

    if reseller.id != user.reseller_id {
        return Err(anyhow::anyhow!("the reseller id doesn't match"));
    }
    Ok(reseller)
}

pub async fn products(
    db: web::Data<FirestoreDb>,
    req: HttpRequest,
    pagination: web::Query<Pagination>,
    api_key: web::Query<APIKey>,
    _: RequestId,
) -> Result<HttpResponse, APIError> {
    let token = get_firebase_token(&req);

    let token = token.context("No token found")?;

    tracing::info!(token);

    let reseller = check_authorization(api_key.api_key.to_string(), token.to_string(), &db)
        .await
        .map_err(APIError::AuthorizationError)?;
    let products = get_products(&db, &reseller)
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
    req: HttpRequest,
    id: web::Path<Uuid>,
    api_key: web::Query<APIKey>,
    _: RequestId,
) -> Result<HttpResponse, APIError> {
    let token = get_firebase_token(&req);

    let token = token.context("No token found")?;

    let reseller = check_authorization(api_key.api_key.to_string(), token.to_string(), &db).await?;

    let product = get_product(&db, *id, &reseller)
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
