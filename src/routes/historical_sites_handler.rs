use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{helper::date_to_string::{timestamp_to_string, timestamptz_to_string}, models::{historical_sites::{HistoricalSites, HistoricalSitesResponse}, response::ResponseModel}};

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

pub async fn add_historical_sites(conn: Extension<Pool<Postgres>>, req_form_data: Json<HistoricalSites>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("INSERT INTO historical_sites (name, category_id, description, location) VALUES ($1, $2, $3, $4) returning id")
        .bind(&req_form_data.name)
        .bind(&req_form_data.category_id)
        .bind(&req_form_data.description)
        .bind(&req_form_data.location)
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let id: i64 = data.get("id");

    let data = sqlx::query("SELECT * FROM historical_sites WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| HistoricalSitesResponse {
            id: row.get("id"),
            name: row.get("name"),
            category_id: row.get("category_id"),
            description: row.get("description"),
            location:row.get("location"),
            updated_at: timestamptz_to_string(row.get("updated_at")),
            created_at: timestamptz_to_string(row.get("created_at")),
        })
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Historical site created successfully".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::CREATED, Json(result)).into_response())
} 