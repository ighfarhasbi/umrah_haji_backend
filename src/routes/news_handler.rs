use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use rust_decimal::prelude::ToPrimitive;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{helper::date_to_string::timestamptz_to_string, models::{news::News, response::{ResponseMessage, ResponseModel}}};

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

pub async fn add_news(conn: Extension<Pool<Postgres>>, req_form_data: Json<News>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("INSERT INTO news (title, content, image_url, category_id) VALUES ($1, $2, $3, $4) returning id")
       .bind(&req_form_data.title)
       .bind(&req_form_data.content)
       .bind(&req_form_data.image_url)
       .bind(&req_form_data.category_id)
       .fetch_one(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let id: i64 = data.get(0);

    let data = sqlx::query("SELECT * FROM news WHERE id = $1")
        .bind(&id)
        .map(|row: PgRow| News {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            image_url: row.get("image_url"),
            category_id: row.get("category_id"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
        })
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(ResponseModel {
        status: "Success".to_string(),
        message: "News added successfully".to_string(),
        data: Some(&data),
    })).into_response())
}

pub async fn update_news(conn: Extension<Pool<Postgres>>, id: Path<i64>, req_form_data: Json<News>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    sqlx::query("UPDATE news SET title = $1, content = $2, image_url = $3, category_id = $4 WHERE id = $5")
       .bind(&req_form_data.title)
       .bind(&req_form_data.content)
       .bind(&req_form_data.image_url)
       .bind(&req_form_data.category_id)
       .bind(&id.to_i64())
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let data = sqlx::query("SELECT * FROM news WHERE id = $1")
       .bind(&id.to_i64())
       .map(|row: PgRow| News {
        id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            image_url: row.get("image_url"),
            category_id: row.get("category_id"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
       })
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = ResponseModel {
        status: "Success".to_string(),
        message: "News updated successfully".to_string(),
        data: Some(&data),
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn delete_news(conn: Extension<Pool<Postgres>>, id: Path<i64>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("DELETE FROM news WHERE id = $1")
       .bind(&id.to_i64())
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if data.rows_affected() == 0 {
        let response = ResponseMessage {
            status: "Error".to_string(),
            message: "News not found".to_string(),
        };
        return Ok((StatusCode::NOT_FOUND, Json(response)).into_response());
    }

    Ok((StatusCode::OK, Json(ResponseMessage {
        status: "Success".to_string(),
        message: "News deleted successfully".to_string(),
    })).into_response())
}