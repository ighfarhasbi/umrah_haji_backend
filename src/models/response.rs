use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ResponseModel<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValueCekToken {
    pub id: i64,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    pub status: String,
    pub message: String,
}