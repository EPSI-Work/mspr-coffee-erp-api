//use crate::entity::{FirebaseUser, PaginationResponse, Product, Reseller};
use crate::helpers::spawn_app;
use base64::encode;
use chrono::prelude::*;
use erp_api::entity::{FirebaseUser, PaginationResponse, Product};
use fake::Fake;
use fake::Faker;
use uuid::Uuid;

#[tokio::test]
async fn get_products_empty() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, _) = app.setup_database(0).await;

    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;
    let firebase_user = encode(serde_json::to_vec(&firebase_user).unwrap());

    // Act
    let response = app
        .get_products(1, 10, reseller_firestore.api_key, firebase_user)
        .await;

    // Assert
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to get body");
    assert_eq!(body, "{\"metadata\":{\"total_results\":0,\"page_size\":10,\"current_page\":1},\"results\":[],\"links\":{\"previous\":null,\"next\":null}}");
}

#[tokio::test]
async fn get_products_auth_error_cases() {
    // Arrange
    // we just need the reseller_firestore for the last test
    let app = spawn_app().await;
    let (reseller_firestore, _, _) = app.setup_database(0).await;

    struct TestCase<'a> {
        name: &'a str,
        api_key: String,
        firebase_user: String,
        expected_error_message: &'a str,
    }

    let firebase_user_not_existing: FirebaseUser = Faker.fake();
    let firebase_user_not_existing_str =
        encode(serde_json::to_vec(&firebase_user_not_existing).unwrap());

    let test_cases = [
        TestCase {
            name: "firebase user : base64 decode error",
            api_key: Uuid::new_v4().to_string(),
            firebase_user: "=========".to_string(),
            expected_error_message: "base64",
        },
        TestCase {
            name: "firebase user : utf8 decode error",
            api_key: Uuid::new_v4().to_string(),
            firebase_user: "AJoSmg==".to_string(),
            expected_error_message: "UTF-8",
        },
        TestCase {
            name: "firebase user : failed parsing",
            api_key: Uuid::new_v4().to_string(),
            firebase_user: "ewogICJpc3MiOiAiaHR0cHM6Ly9zZWN1cmV0b2tlbi5nb29nbGUuY29tL21zcHItZXBzaS1jb2ZmZWUiLAogICJhdWQiOiAibXNwci1lcHNpLWNvZmZlZSIsCiAgImF1dGhfdGltZSI6IDE2Nzg5NTg3MjgsCn0=".to_string(),
            expected_error_message: "Failed to parse firebase json",
        },
        TestCase {
            name: "reseller not found",
            api_key: Uuid::new_v4().to_string(),
            firebase_user: firebase_user_not_existing_str.clone(),
            expected_error_message: "No reseller found",
        },
        TestCase {
            name: "user not found",
            api_key: reseller_firestore.api_key,
            firebase_user: firebase_user_not_existing_str,
            expected_error_message: "No user found",
        },
    ];

    for test_case in test_cases {
        // Act
        let response = app
            .get_products(
                1,
                10,
                test_case.api_key.clone(),
                test_case.firebase_user.clone(),
            )
            .await;

        // Assert
        assert_eq!(response.status(), 401, "{}", test_case.name);
        let body = response.text().await.expect("Failed to get body");
        assert_eq!(true, body.contains(test_case.expected_error_message));
    }
}

#[tokio::test]
async fn get_products() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, products_firestore) = app.setup_database(2).await;

    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;
    let firebase_user = encode(serde_json::to_vec(&firebase_user).unwrap());

    // Act
    let response = app
        .get_products(1, 10, reseller_firestore.api_key, firebase_user)
        .await;

    // Assert
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to get body");
    let mut expected_products = vec![products_firestore[0].clone(), products_firestore[1].clone()];
    let mut res_products =
        serde_json::from_str::<PaginationResponse<Product>>(&body).expect("Failed");

    assert_eq!(
        res_products.results.sort_by(|a, b| a.id.cmp(&b.id)),
        expected_products.sort_by(|a, b| a.id.cmp(&b.id))
    );
}

#[tokio::test]
async fn get_one_product() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, products_firestore) = app.setup_database(1).await;

    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;
    let firebase_user = encode(serde_json::to_vec(&firebase_user).unwrap());

    let mut product = products_firestore[0].clone();

    // Act
    let response = app
        .get_product(
            product.id.to_string(),
            reseller_firestore.api_key,
            firebase_user,
        )
        .await;

    // Assert
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to get body");

    let mut product_return = serde_json::from_str::<Product>(&body).expect("Failed");

    product_return.created_at = product_return.created_at.with_nanosecond(6).unwrap();
    product.created_at = product.created_at.with_nanosecond(6).unwrap();

    assert_eq!(product_return, product);
}

#[tokio::test]
async fn get_one_product_not_found() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, _) = app.setup_database(0).await;

    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;
    let firebase_user = encode(serde_json::to_vec(&firebase_user).unwrap());

    // Act
    let response = app
        .get_product(
            Uuid::new_v4().to_string(),
            reseller_firestore.api_key,
            firebase_user,
        )
        .await;

    // Assert
    assert_eq!(response.status(), 400);
    let body = response.text().await.expect("Failed to get body");
    assert_ne!(
        body.len(),
        0,
        "There should be a message send to the client"
    );
}

#[tokio::test]
async fn get_one_product_no_valid_uuid_provided() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, _) = app.setup_database(0).await;

    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;
    let firebase_user = encode(serde_json::to_vec(&firebase_user).unwrap());

    // Act
    let response = app
        .get_product(
            "156e0b6c-3eb1-46a2-8a23-48a1251ef34143".to_string(),
            reseller_firestore.api_key,
            firebase_user,
        )
        .await;

    // Assert
    assert_eq!(response.status(), 404);
    let body = response.text().await.expect("Failed to get body");
    assert_ne!(
        body.len(),
        0,
        "There should be a message send to the client"
    );
}
