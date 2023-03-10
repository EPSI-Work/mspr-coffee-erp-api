use fake::{Dummy, Fake};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Dummy)]
pub struct FirebaseUser {
    pub iss: String,
    pub aud: String,
    pub auth_time: i64,
    pub user_id: String,
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub email: String,
    pub email_verified: bool,
    pub firebase: Firebase,
}

#[derive(Serialize, Deserialize, Dummy)]
pub struct Firebase {
    pub identities: Identities,
    sign_in_provider: String,
}

#[derive(Serialize, Deserialize, Dummy)]
pub struct Identities {
    pub email: Vec<String>,
}
