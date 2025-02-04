use serde::Serialize;

#[derive(Serialize)]
pub struct HistoricalSites {
    pub id: i64,
    pub name: String,
    pub location: String,
    pub description: Option<String>,
    pub historical_significance: Option<String>,
    pub image_url: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub established_date: Option<String>,
    pub visitor_info: Option<String>,
    pub category_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}