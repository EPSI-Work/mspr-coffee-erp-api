use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq, Dummy)]
pub struct Reseller {
    pub id: Uuid,
    pub name: String,
    pub api_key: String,
}
