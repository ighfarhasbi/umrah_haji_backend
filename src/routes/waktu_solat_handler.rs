use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::models::{
    response::ResponseModel,
    waktu_solat::{ApiResponse, QueryParams},
};

pub async fn get_prayer_times(Query(params): Query<QueryParams>) -> Result<Response, (StatusCode, String)> {
    let url = format!("https://waktu-sholat.vercel.app/prayer?latitude={}&longitude={}",
        params.latitude, params.longitude
    );

    let response = reqwest::get(&url).await;
    if let Err(_) = response {
        return Ok(
            (StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseModel::<()> {
                status: "Error".to_string(),
                message: "Gagal memanggil API".to_string(),
                data: None,
            })).into_response()
        );
    }

    let response = response.unwrap();
    let response_json: Result<ApiResponse, _> = response.json().await;
    if let Err(_) = response_json {
        return Ok(
            (StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseModel::<()> {
                status: "Error".to_string(),
                message: "Gagal mengurai respons API".to_string(),
                data: None,
            })).into_response()
        );
    }

    let response_json = response_json.unwrap();

    let result = ResponseModel {
        status: "Success".to_string(),
        message: "Berhasil mendapatkan data waktu sholat".to_string(),
        data: Some(response_json),
    };
    Ok((StatusCode::OK, Json(result)).into_response())
}
