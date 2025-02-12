use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Faqs {
    pub id: Option<i64>,
    pub category: i32,
    pub orders: i32,
    pub question: String,
    pub answer: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub type_menu: Option<String>
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub type_menu: String
}