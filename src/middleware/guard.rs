use std::{future::Future, pin::Pin};

use axum::{
    extract::Extension, http::{HeaderValue, Request, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json
    };
use axum_extra::headers::{authorization::Bearer, Authorization, Header, HeaderMapExt};
use axum::body::Body;
use sqlx::Row;
use sqlx::{postgres::PgRow, Pool, Postgres};

use crate::{jsonwebtoken::jwt::{is_valid, refresh_access_token}, models::response::{ResponseModel, ValueCekToken}};

// Definisikan tipe untuk fungsi yang akan dipanggil secara rekursif
type _GuardRouteFuture = Pin<Box<dyn Future<Output = Result<Response, (StatusCode, String)>> + Send>>;

pub async fn guard_route (
    conn: Extension<Pool<Postgres>>,
    mut request: Request<Body>,
    next: Next
) -> Result<Response, (StatusCode, String) >{

    let conn2 = conn.clone();
    let conn3 = conn.clone();
    // ambil token dulu
    let token = request
    .headers().typed_get::<Authorization<Bearer>>()
        .ok_or((StatusCode::BAD_REQUEST, "token tidak ditemukan".to_string()))?
        .token()
        .to_owned();
    // println!("ini tokennya, {:?}", token);

    let extension = request.extensions_mut();
    let user = cek_token(conn, token.clone()).await;
    let ref_token_str: String;
    match user {
        Ok(result) => {
            // println!("ini usernya dari guard = {:?}", result);
            if result.access_token == "" {
                // return Err((StatusCode::UNAUTHORIZED, "Token tidak valid".to_string())); // jika token tidak valid maka akan berhenti disini
                return Ok((StatusCode::UNAUTHORIZED, Json(ResponseModel {
                    status: StatusCode::UNAUTHORIZED.to_string(),
                    message: "Token tidak valid".to_string(),
                    data: Some("".to_string()),
                })).into_response());
            } else {
                ref_token_str = result.refresh_token.clone();
                extension.insert(result);
            }
        }
        Err(_) => {
            return Err((StatusCode::UNAUTHORIZED, "Token tidak valid, harap login".to_string()));
        }
    }

    // cek apakah token valid atau tidak (expired atau belum)
    let dur = is_valid(&token);
    // println!("ini dur = {:?}", dur);
    match dur {
        Ok(_) => {
            // tidak melakukan apapa jika token ada
        }
        Err(_e) => {
            let new_acc_token = refresh_access_token(&ref_token_str);
            match new_acc_token {
                Ok(new_access_token) => { 
                    // println!("access token baru => {:?}", new_acc_token);
                    let _ = update_token(conn2, new_access_token.clone(), token);

                    // Update token di header request
                    let headers = request.headers_mut();
                    headers.insert(
                        Authorization::<Bearer>::name(),
                        HeaderValue::from_str(&format!("Bearer {}", new_access_token)).unwrap(),
                    );

                    // Panggil ulang guard_route dengan token yang diperbarui
                    return Box::pin(guard_route(conn3, request, next)).await;

                }
                Err(_) => {return Err((StatusCode::UNAUTHORIZED, "Token expired".to_string()));}  
            }
        }
    }
    let response = next.run(request).await;
    Ok(response)
}

async fn cek_token(conn: Extension<Pool<Postgres>>, token: String) -> Result<ValueCekToken, StatusCode> {
    
    // cek token dengan yang ada di db
    let conn = conn.0;
    let result = sqlx::query("SELECT id, access_token, refresh_token FROM token_user WHERE access_token = $1")
    .bind(&token)
    .map(|row: PgRow| {
        ValueCekToken { 
            id: row.get("id"),
            access_token: row.get("access_token"),
            refresh_token: row.get("refresh_token"),
        }
    })
    .fetch_optional(&conn).await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR))?;

    // inisisasi struct kosong
    let user_result: ValueCekToken;
    // mengubah tipe data dari MappedRows pada var user_id_result menjadi struck ResponseUser
    if let Some(user) = result {
        if user.access_token != token { 
            return Err(StatusCode::UNAUTHORIZED);
        } else {
            user_result = user;
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    Ok(ValueCekToken {
        id: user_result.id,
        access_token: user_result.access_token,
        refresh_token: user_result.refresh_token
    })
}

async fn update_token (conn: Extension<Pool<Postgres>>, new_token: String, old_token: String) -> Result<(), (StatusCode, String)> {
    let conn = conn.0;
    sqlx::query("UPDATE token_user SET access_token = $1 WHERE access_token = $2")
       .bind(&new_token)
       .bind(&old_token)
       .execute(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("ini token baru = {}", new_token);
    Ok(())
}