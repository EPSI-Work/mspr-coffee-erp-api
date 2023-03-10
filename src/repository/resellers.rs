use crate::entity::Reseller;
use firestore::errors::FirestoreError;
use firestore::struct_path::path;
use firestore::FirestoreDb;
use futures::stream::BoxStream;
use futures::StreamExt;

pub const COLLECTION_NAME_RESELLERS: &str = "resellers";

pub async fn get_reseller(
    db: &FirestoreDb,
    api_key: &String,
) -> Result<Option<Reseller>, FirestoreError> {
    let reseller: BoxStream<Reseller> = db
        .fluent()
        .select()
        .from(COLLECTION_NAME_RESELLERS)
        .filter(|q| q.for_all([q.field(path!(Reseller::api_key)).eq(api_key)]))
        .obj()
        .stream_query()
        .await?;
    let as_vec: Vec<Reseller> = reseller.collect().await;

    // TO DO handle if two element where found
    if as_vec.is_empty() {
        return Ok(None);
    }
    let first_element = as_vec[0].clone();
    Ok(Some(first_element))
}
