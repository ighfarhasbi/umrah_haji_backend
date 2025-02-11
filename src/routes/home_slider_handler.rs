use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use rust_decimal::prelude::ToPrimitive;
use sqlx::{postgres::PgRow, Pool, Postgres, Row};


use crate::{helper::date_to_string::timestamptz_to_string, models::{home_sliders::HomeSliders, response::{ResponseMessage, ResponseModel}}};

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

pub async fn add_slider(conn: Extension<Pool<Postgres>>, req_form_data: Json<HomeSliders>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("INSERT INTO home_sliders (type, title, descs, image, status, orders) VALUES ($1, $2, $3, $4, $5, $6) returning id")
       .bind(&req_form_data.r#type)
       .bind(&req_form_data.title)
       .bind(&req_form_data.descs)
       .bind(&req_form_data.image)
       .bind(&req_form_data.status)
       .bind(&req_form_data.orders)
       .fetch_one(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let id: i64 = data.get("id");

    let data = sqlx::query("SELECT * FROM home_sliders WHERE id = $1")
        .bind(id)
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
        .fetch_one(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Slider added successfully".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::CREATED, Json(result)).into_response())
}

pub async fn update_slider(conn: Extension<Pool<Postgres>>, id: Path<i64>, req_form_data: Json<HomeSliders>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    sqlx::query("UPDATE home_sliders SET type = $1, title = $2, descs = $3, image = $4, status = $5, orders = $6 WHERE id = $7")
       .bind(&req_form_data.r#type)
       .bind(&req_form_data.title)
       .bind(&req_form_data.descs)
       .bind(&req_form_data.image)
       .bind(&req_form_data.status)
       .bind(&req_form_data.orders)
       .bind(&id.to_i64())
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let data = sqlx::query("SELECT * FROM home_sliders WHERE id = $1")
        .bind(id.to_i64())
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
        .fetch_one(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Slider updated successfully".to_string(),
        data: Some(&data),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}

pub async fn delete_slider(conn: Extension<Pool<Postgres>>, id: Path<i64>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;

    let data = sqlx::query("DELETE FROM home_sliders WHERE id = $1")
       .bind(&id.to_i64())
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if data.rows_affected() == 0 {
        let response = ResponseMessage {
            status: "Error".to_string(),
            message: "Slider not found".to_string(),
        };
        return Ok((StatusCode::NOT_FOUND, Json(response)).into_response());
    }

    let result = ResponseMessage {
        status: "Success".to_string(),
        message: "Slider deleted successfully".to_string(),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}
