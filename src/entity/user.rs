use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialOrd, PartialEq, Dummy)]
pub struct User {
    pub firebase_id: String,
    pub reseller_id: Uuid,
}
