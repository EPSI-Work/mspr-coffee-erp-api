use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq, Dummy)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub subtitle: String,
    pub details: Option<Detail>,
    pub stock: i64,
    #[serde(with = "firestore::serialize_as_timestamp")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Dummy)]
pub struct Detail {
    pub price: f64,
    pub description: String,
    pub color: String,
}
