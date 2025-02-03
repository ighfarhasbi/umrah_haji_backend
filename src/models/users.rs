use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReqLoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<String>,
    pub two_factor_secret: Option<String>,
    pub two_factor_recovery_codes: Option<String>,
    pub two_factor_confirmed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub roles: String,
}

#[derive(Serialize)]
pub struct ResponseLogin {
    pub user: Option<User>,
    pub token: String,
}

#[derive(Deserialize)]
pub struct ReqSignIn {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub roles: String,
}

#[derive(Serialize)]
pub struct ResponseRegister {
    pub status: String,
    pub message: String,
    pub data: Option<RegUser>,
}

#[derive(Serialize)]
pub struct RegUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub roles: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub id: i64,
}
