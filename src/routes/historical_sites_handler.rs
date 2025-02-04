use axum::{http::StatusCode,response::{IntoResponse, Response}, Extension, Json};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{helper::date_to_string::{timestamp_to_string, timestamptz_to_string}, models::{historical_sites::HistoricalSites, response::ResponseModel}};

pub async fn get_historical_sites(conn: Extension<Pool<Postgres>>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("select * from historical_sites")
        .map(|row: PgRow| HistoricalSites {
            id: row.get("id"),
            name: row.get("name"),
            location: row.get("location"),
            description: row.get("description"),
            historical_significance: row.get("historical_significance"),
            image_url: row.get("image_url"),
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            established_date: timestamp_to_string(row.get("established_date")),
            visitor_info: row.get("visitor_info"),
            category_id: row.get("category_id"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
        })
        .fetch_all(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Get all historical sites".to_string(),
        data: Some(&data),
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}