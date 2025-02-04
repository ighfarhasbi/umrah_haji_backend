use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};


use crate::{helper::date_to_string::timestamptz_to_string, models::{home_sliders::HomeSliders, response::ResponseModel}};

pub async fn get_sliders(conn: Extension<Pool<Postgres>>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("select * from home_sliders")
        .map(|row: PgRow| HomeSliders {
            id: row.get("id"),
            r#type: row.get("type"),
            title: row.get("title"),
            descs: row.get("descs"),
            image: row.get("image"),
            status: row.get("status"),
            orders: row.get("orders"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
        })
        .fetch_all(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Get all sliders".to_string(),
        data: Some(&data),
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}
