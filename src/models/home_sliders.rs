use serde::Serialize;

#[derive(Serialize)]
pub struct HomeSliders {
    pub id: i64,
    #[serde(rename = "type")]
    pub r#type: String,
    pub title: Option<String>,
    pub descs: Option<String>,
    pub image: Option<String>,
    pub status: i32,
    pub orders: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}