use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{helper::date_to_string::timestamptz_to_string, models::{news::News, response::ResponseModel}};

pub async fn get_news(conn: Extension<Pool<Postgres>>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("SELECT * FROM news")
        .map(|row: PgRow| News {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            image_url: row.get("image_url"),
            category_id: row.get("category_id"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
        })
        .fetch_all(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Get all news".to_string(),
        data: Some(&data),
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}