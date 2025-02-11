use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HomeSliders {
    pub id: Option<i64>,
    #[serde(rename = "type")] // Karena "type" merupakan salah satu variabel yg ada dalam rust. Jadi perlu modif agar sama dengan yg ada di db
    pub r#type: String,
    pub title: Option<String>,
    pub descs: Option<String>,
    pub image: Option<String>,
    pub status: i32,
    pub orders: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}