use crate::entity::{generate_pagination_response, Product};
use crate::error::APIError;
use crate::repository::get_products;
use crate::repository::get_reseller;
use crate::repository::{get_product, get_user};
use crate::startup::CloudFunction;
use actix_web::http::header::ContentType;
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse};
use anyhow::Context;
use fake::{Dummy, Fake};
use firestore::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string};
use tracing_actix_web::RequestId;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct FirebaseUser {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Dummy)]
pub struct User {
    pub uid: String,
    pub email: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    pub disabled: bool,
    #[serde(rename = "tokensValidAfterTime")]
    pub tokens_valid_after_time: String,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
}

#[derive(Deserialize, Debug)]
pub struct APIKey {
    pub api_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VerifyFirebaseToken {
    #[serde(rename = "firebaseToken")]
    pub firebase_token: String,
}

fn get_firebase_token(req: &HttpRequest) -> Option<&str> {
    req.headers()
        .get("x-apigateway-api-userinfo")
        .map(|value| value.to_str().ok())
        .unwrap_or_else(|| None)
}

pub async fn products(
    db: web::Data<FirestoreDb>,
    req: HttpRequest,
    pagination: web::Query<Pagination>,
    api_key: web::Query<APIKey>,
    cloud_function: web::Data<CloudFunction>,
    _: RequestId,
) -> Result<HttpResponse, APIError> {
    let token = get_firebase_token(&req);

    if token.is_none() {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("token firebase not found"));
    }

    let firebase_credentials = VerifyFirebaseToken {
        firebase_token: token.unwrap().to_string(),
    };

    // verify firebase token
    let response = reqwest::Client::new()
        .post(format!("{}/api/auth/v1/verifyToken", cloud_function.host))
        .json(&firebase_credentials)
        .send()
        .await
        .context("Failed to verify firebase token")?;

    let body = response
        .text()
        .await
        .context("Failed to get body from firebase token")?;

    let firebase_user =
        serde_json::from_str::<FirebaseUser>(&body).context("Failed to parse json")?;

    // check if the reseller with the given api key exist
    let reseller = get_reseller(&db, &api_key.api_key)
        .await
        .context("Failed to get products from database.")?;

    if reseller.is_none() {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("not reseller found with this api_key"));
    }

    tracing::info!("Reseller with the api key is found");

    let reseller = reseller.unwrap();

    // check if the user has a reseller
    let user = get_user(&db, &firebase_user.user.uid, &reseller)
        .await
        .context("Failed to get products from database.")?;

    if user.is_none() {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("user has not associtated reseller"));
    }

    tracing::info!("User is found");

    if reseller.id != user.unwrap().reseller_id {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("bad api key"));
    }

    // check if the user reseller_id and the reseller_id for the given api_key match
    // if(reseller.unwrap().)

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
    cloud_function: web::Data<CloudFunction>,
    _: RequestId,
) -> Result<HttpResponse, APIError> {
    let token = get_firebase_token(&req);

    if token.is_none() {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("token firebase not found"));
    }

    let firebase_credentials = VerifyFirebaseToken {
        firebase_token: token.unwrap().to_string(),
    };

    // verify firebase token
    let response = reqwest::Client::new()
        .post(format!("{}/api/auth/v1/verifyToken", cloud_function.host))
        .json(&firebase_credentials)
        .send()
        .await
        .context("Failed to verify firebase token")?;

    let body = response
        .text()
        .await
        .context("Failed to get body from firebase token")?;

    let firebase_user =
        serde_json::from_str::<FirebaseUser>(&body).context("Failed to parse json")?;

    // check if the reseller with the given api key exist
    let reseller = get_reseller(&db, &api_key.api_key)
        .await
        .context("Failed to get products from database.")?;

    if reseller.is_none() {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("not reseller found with this api_key"));
    }

    let reseller = reseller.unwrap();

    // check if the user has a reseller
    let user = get_user(&db, &firebase_user.user.uid, &reseller)
        .await
        .context("Failed to get products from database.")?;

    if user.is_none() {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("user has not associtated reseller"));
    }

    if reseller.id != user.unwrap().reseller_id {
        return Ok(HttpResponse::BadRequest()
            .content_type(ContentType::json())
            .body("bad api key"));
    }

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
