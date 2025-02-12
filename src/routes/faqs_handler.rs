use axum::{extract::{Path, Query}, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use rust_decimal::prelude::ToPrimitive;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};

use crate::{helper::date_to_string::timestamptz_to_string, models::{faqs::{Faqs, QueryParam}, response::{ResponseMessage, ResponseModel}}};

pub async fn get_faqs(conn: Extension<Pool<Postgres>>, req_query: Query<QueryParam>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("SELECT * FROM faqs WHERE type_menu = $1")
        .bind(&req_query.type_menu)
        .map(|row: PgRow| Faqs {
            id: row.get("id"),
            category: row.get("category"),
            orders: row.get("orders"),
            question: row.get("question"),
            answer: row.get("answer"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
            type_menu: row.get("type_menu"),
        })
        .fetch_all(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Cek apakah data kosong
    if data.is_empty() {
        let result = ResponseModel::<()> {
            status: StatusCode::NOT_FOUND.to_string(),
            message: "Data tidak ditemukan".to_string(),
            data: None,
        };
        return Ok((StatusCode::NOT_FOUND, Json(result)).into_response());
    }

    let result = ResponseModel {
        status: StatusCode::OK.to_string(),
        message: "Sukses".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn add_faqs(conn: Extension<Pool<Postgres>>, req_query: Query<QueryParam>, req_form_data: Json<Faqs>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("INSERT INTO faqs (category, orders, question, answer, type_menu) VALUES ($1, $2, $3, $4, $5) returning id")
       .bind(&req_form_data.category)
       .bind(&req_form_data.orders)
       .bind(&req_form_data.question)
       .bind(&req_form_data.answer)
       .bind(&req_query.type_menu)
       .fetch_one(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let id: i64 = data.get("id");

    let data = sqlx::query("SELECT * FROM faqs WHERE id = $1")
        .bind(id)
        .map(|row: PgRow| Faqs {
            id: row.get("id"),
            category: row.get("category"),
            orders: row.get("orders"),
            question: row.get("question"),
            answer: row.get("answer"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
            type_menu: row.get("type_menu"),
        })
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = ResponseModel {
        status: StatusCode::CREATED.to_string(),
        message: "Data berhasil ditambahkan".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::CREATED, Json(result)).into_response())
}

pub async fn update_faqs(conn: Extension<Pool<Postgres>>, req_query: Query<QueryParam>, req_id: Path<i64>, req_form_data: Json<Faqs>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("UPDATE faqs SET category = $1, orders = $2, question = $3, answer = $4 WHERE id = $5 AND type_menu = $6")
       .bind(&req_form_data.category)
       .bind(&req_form_data.orders)
       .bind(&req_form_data.question)
       .bind(&req_form_data.answer)
       .bind(&req_id.to_i64())
       .bind(&req_query.type_menu)
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if data.rows_affected() == 0 {
        let result = ResponseModel::<()> {
            status: StatusCode::NOT_FOUND.to_string(),
            message: "Data tidak ditemukan".to_string(),
            data: None,
        };
        return Ok((StatusCode::NOT_FOUND, Json(result)).into_response());
    }

    let data = sqlx::query("SELECT * FROM faqs WHERE id = $1")
       .bind(req_id.to_i64())
       .map(|row: PgRow| Faqs {
            id: row.get("id"),
            category: row.get("category"),
            orders: row.get("orders"),
            question: row.get("question"),
            answer: row.get("answer"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
            type_menu: row.get("type_menu"),
        })
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let result = ResponseModel {
        status: StatusCode::OK.to_string(),
        message: "Data berhasil diupdate".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn delete_faqs(conn: Extension<Pool<Postgres>>, req_query: Query<QueryParam>, req_id: Path<i64>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("DELETE FROM faqs WHERE id = $1 AND type_menu = $2")
       .bind(&req_id.to_i64())
       .bind(&req_query.type_menu)
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if data.rows_affected() == 0 {
        let result = ResponseMessage {
            status: StatusCode::NOT_FOUND.to_string(),
            message: "Data tidak ditemukan".to_string(),
        };
        return Ok((StatusCode::NOT_FOUND, Json(result)).into_response());
    }

    let response = ResponseMessage {
        status: "Success".to_string(),
        message: "Faqs deleted successfully".to_string(),
    };
    Ok((StatusCode::OK, Json(response)).into_response())
}