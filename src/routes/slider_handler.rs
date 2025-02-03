use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};

use crate::models::response::{ResponseModel, ValueCekToken};

pub async fn get_sliders(data: Extension<ValueCekToken>) -> Result<Response, (StatusCode, String)> {
    let val = ValueCekToken {
        id: data.id.clone(),
        access_token: data.access_token.clone(),
        refresh_token: data.refresh_token.clone(), 
    };
    let res = ResponseModel {
        status: StatusCode::OK.to_string(),
        message: "Sukses".to_string(),
        data: Some(val)
    };
    Ok((StatusCode::OK, Json(res)).into_response())
}
