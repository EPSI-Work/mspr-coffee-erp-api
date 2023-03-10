use crate::helpers::spawn_app;
use base64::encode;
use chrono::prelude::*;
use erp_api::entity::FirebaseUser;
use erp_api::entity::PaginationResponse;
use erp_api::entity::Product;
use fake::Fake;
use fake::Faker;
use uuid::Uuid;

#[tokio::test]
async fn get_products_empty() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, _) = app.setup_database(0).await;

    // Make sure the uid return from the mock cloud functions exist in the firestore database
    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;

    let firebase_user = serde_json::to_vec(&firebase_user).unwrap();
    let firebase_user = encode(&firebase_user);

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
async fn get_products() {
    // Arrange
    let app = spawn_app().await;
    let (reseller_firestore, user_firestore, products_firestore) = app.setup_database(2).await;

    // Make sure the uid return from the mock cloud functions exist in the firestore database
    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;

    let firebase_user = serde_json::to_vec(&firebase_user).unwrap();
    let firebase_user = encode(&firebase_user);

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

    // Make sure the uid return from the mock cloud functions exist in the firestore database
    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;

    let firebase_user = serde_json::to_vec(&firebase_user).unwrap();
    let firebase_user = encode(&firebase_user);

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

    // Make sure the uid return from the mock cloud functions exist in the firestore database
    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;

    let firebase_user = serde_json::to_vec(&firebase_user).unwrap();
    let firebase_user = encode(&firebase_user);

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

    // Make sure the uid return from the mock cloud functions exist in the firestore database
    let mut firebase_user: FirebaseUser = Faker.fake();
    firebase_user.user_id = user_firestore.firebase_id;

    let firebase_user = serde_json::to_vec(&firebase_user).unwrap();
    let firebase_user = encode(&firebase_user);

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
