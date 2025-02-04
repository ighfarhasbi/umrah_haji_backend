use serde::Serialize;

#[derive(Serialize)]
pub struct Doas {
    pub id: i64,
    pub numb_order: Option<i32>,
    pub title: String,
    pub content: String,
    pub audio_url: Option<String>,
    pub category_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub image_url: Option<String>,
}