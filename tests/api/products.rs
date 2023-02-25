use crate::helpers::spawn_app;
use chrono::prelude::*;
use erp_api::entity::Product;
use fake::Fake;
use fake::Faker;
use uuid::Uuid;

const COLLECTION_NAME: &str = "products";

#[tokio::test]
async fn get_products_empty() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_products().await;

    // Assert
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to get body");
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn get_products() {
    // Arrange
    let app = spawn_app().await;

    let product1: Product = Faker.fake();
    let product2: Product = Faker.fake();

    let _: Product = app
        .db
        .fluent()
        .insert()
        .into(COLLECTION_NAME)
        .document_id(&product1.id.to_string())
        .object(&product1)
        .execute()
        .await
        .expect("Failed to insert product in test firestore database");

    let _: Product = app
        .db
        .fluent()
        .insert()
        .into(COLLECTION_NAME)
        .document_id(&product2.id.to_string())
        .object(&product2)
        .execute()
        .await
        .expect("Failed to insert product in test firestore database");

    // Act
    let response = app.get_products().await;

    // Assert
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to get body");
    let mut expected_products = vec![product1.clone(), product2.clone()];
    let mut res_products = serde_json::from_str::<Vec<Product>>(&body).expect("Failed");

    assert_eq!(
        res_products.sort_by(|a, b| a.id.cmp(&b.id)),
        expected_products.sort_by(|a, b| a.id.cmp(&b.id))
    );

    // Clean
    app.db
        .fluent()
        .delete()
        .from(COLLECTION_NAME)
        .document_id(&product1.id.to_string())
        .execute()
        .await
        .expect("Failed to delete product");
    app.db
        .fluent()
        .delete()
        .from(COLLECTION_NAME)
        .document_id(&product2.id.to_string())
        .execute()
        .await
        .expect("Failed to delete product");
}

#[tokio::test]
async fn get_one_product() {
    // Arrange
    let app = spawn_app().await;

    let mut product: Product = Faker.fake();

    let _: Product = app
        .db
        .fluent()
        .insert()
        .into(COLLECTION_NAME)
        .document_id(&product.id.to_string())
        .object(&product)
        .execute()
        .await
        .expect("Failed to insert product in test firestore database");

    // Act
    let response = app.get_product(product.id.to_string()).await;

    // Assert
    assert_eq!(response.status(), 200);
    let body = response.text().await.expect("Failed to get body");

    let mut product_return = serde_json::from_str::<Product>(&body).expect("Failed");

    product_return.created_at = product_return.created_at.with_nanosecond(6).unwrap();
    product.created_at = product.created_at.with_nanosecond(6).unwrap();

    assert_eq!(product_return, product);

    // Clean
    app.db
        .fluent()
        .delete()
        .from(COLLECTION_NAME)
        .document_id(&product.id.to_string())
        .execute()
        .await
        .expect("Failed to delete product");
}

#[tokio::test]
async fn get_one_product_not_found() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_product(Uuid::new_v4().to_string()).await;

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

    // Act
    let response = app
        .get_product("156e0b6c-3eb1-46a2-8a23-48a1251ef34143".to_string())
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
