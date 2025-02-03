use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Categories {
    pub id: i64,
    pub name: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
