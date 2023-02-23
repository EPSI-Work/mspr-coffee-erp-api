use crate::helpers::spawn_app;
use chrono::Utc;
use erp_api::routes::Detail;
use erp_api::routes::Product;
use uuid::Uuid;

const COLLECTION_NAME: &'static str = "products";

#[tokio::test]
async fn get_one_product() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let product = Product {
        id: Uuid::new_v4(),
        name: "lkdjsf".to_string(),
        stock: 434,
        created_at: Utc::now(),
        details: Some(Detail {
            price: 43.43,
            description: "dsf".to_string(),
            color: "lsdkf".to_string(),
        }),
    };
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
    let response = client
        .get(&format!(
            "{}/products/{}",
            &app.address,
            &product.id.to_string()
        ))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());

    // Clean
    app.db
        .fluent()
        .delete()
        .from(COLLECTION_NAME)
        .document_id(&product.id.to_string())
        .execute()
        .await
        .expect("Failed to delete product");

    //assert_eq!(response, response.content_length());
}
