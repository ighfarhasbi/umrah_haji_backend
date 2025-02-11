use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct News {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub image_url: Option<String>,
    pub category_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}