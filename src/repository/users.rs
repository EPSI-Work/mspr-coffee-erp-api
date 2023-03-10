use crate::entity::{Reseller, User};
use crate::repository::COLLECTION_NAME_RESELLERS;
use firestore::errors::FirestoreError;
use firestore::struct_path::path;
use firestore::FirestoreDb;
use futures::stream::BoxStream;
use futures::StreamExt;

const COLLECTION_NAME: &str = "users";

pub async fn get_user(
    db: &FirestoreDb,
    firebase_id: &String,
    reseller: &Reseller,
) -> Result<Option<User>, FirestoreError> {
    let parent_path = db.parent_path(COLLECTION_NAME_RESELLERS, reseller.id.to_string())?;

    let users: BoxStream<User> = db
        .fluent()
        .select()
        .from(COLLECTION_NAME)
        .filter(|q| q.for_all([q.field(path!(User::firebase_id)).eq(firebase_id)]))
        .parent(&parent_path)
        .obj()
        .stream_query()
        .await?;
    let as_vec: Vec<User> = users.collect().await;

    if as_vec.is_empty() {
        return Ok(None);
    }
    let first_element = as_vec[0].clone();
    Ok(Some(first_element))
}
