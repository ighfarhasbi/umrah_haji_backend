use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{helper::date_to_string::timestamptz_to_string, models::{doas::{Doas, DoasResponse}, response::ResponseModel}};

pub async fn get_doas(conn: Extension<Pool<Postgres>>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("SELECT * FROM doas")
        .map(|row: PgRow| Doas {
            id: row.get("id"),
            numb_order: row.get("numb_order"),
            title: row.get("title"),
            content: row.get("content"),
            audio_url: row.get("audio_url"),
            category_id: row.get("category_id"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
            image_url: row.get("image_url"),
        })
        .fetch_all(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Get all doas".to_string(),
        data: Some(&data),
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn add_doas(conn: Extension<Pool<Postgres>>, req_form_data: Json<Doas>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("INSERT INTO doas (numb_order, title, content, audio_url, category_id, image_url) VALUES ($1, $2, $3, $4, $5, $6) returning id")
        .bind(&req_form_data.numb_order)
        .bind(&req_form_data.title)
        .bind(&req_form_data.content)
        .bind(&req_form_data.audio_url)
        .bind(&req_form_data.category_id)
        .bind(&req_form_data.image_url)
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let id: i64 = data.get("id");

    let data = sqlx::query("SELECT * FROM doas WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| DoasResponse {
            id: row.get("id"),
            title: row.get("title"),
            category_id: row.get("category_id"),
            content: row.get("content"),
            updated_at: timestamptz_to_string(row.get("updated_at")),
            created_at: timestamptz_to_string(row.get("created_at")),
        })
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Doa created successfully".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::CREATED, Json(result)).into_response())
} 