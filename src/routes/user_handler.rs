use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use sqlx::Row;
use sqlx::{postgres::PgRow, Pool, Postgres};

use crate::{
    helper::date_to_string::{timestamp_to_string, timestamptz_to_string}, jsonwebtoken::jwt::{create_access_token, create_refresh_token}, models::{
        // categories::Categories,
        response::ResponseModel, users::{RegUser, ReqLoginUser, ReqSignIn, ResponseLogin, User}
    }
};

// pub async fn get_user(conn: Extension<Pool<Postgres>>) -> Result<Response, (StatusCode, String)> {
//     let conn = conn.0;

//     let data = sqlx::query("select * from categories")
//         .map(|row: PgRow| Categories {
//             id: row.get("id"),
//             name: row.get("name"),
//             created_at: timestamp_to_string(row.get("created_at")),
//             updated_at: timestamp_to_string(row.get("updated_at")),
//         })
//         .fetch_all(&conn)
//         .await
//         .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

//     let result = ResponseModel {
//         status: StatusCode::OK.to_string(),
//         message: "Sukses".to_string(),
//         data: Some(&data),
//     };

//     Ok((StatusCode::OK, Json(result)).into_response())
// }

pub async fn login_user(conn: Extension<Pool<Postgres>>, req_login: Json<ReqLoginUser>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0; // membuat koneksi ke database

    // get email dan pass
    let data_req = sqlx::query("SELECT id, email, password FROM users WHERE email = $1")
        .bind(&req_login.email)
        .fetch_optional(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // verify email dan ambil variable pass
    let password: String;
    let user_id: i64;
    if let Some(row) = data_req {
        password = row.get("password");
        user_id = row.get("id");
    } else {
        return Err((StatusCode::UNAUTHORIZED, "Email atau password salah".to_string()));
    }

    // verify password menggunakan bcrypt
    let pass = bcrypt::verify(req_login.password.clone(), &password);
    match pass {
        Ok(data) => {
            if data == false {
                return Err((StatusCode::UNAUTHORIZED, "Email atau password salah".to_string()));
            } else {
                // ambil data dari tabel user setelah login
                let data = sqlx::query("select id,name,email,email_verified_at,two_factor_secret,two_factor_recovery_codes,two_factor_confirmed_at,created_at,updated_at,phone,address,roles from users where email = $1")
                    .bind(&req_login.email)
                    .map(|row: PgRow| {
                        User { 
                            id: row.get("id"),
                            name: row.get("name"),
                            email: row.get("email"),
                            email_verified_at: timestamp_to_string(row.get("email_verified_at")),
                            two_factor_secret: row.get("two_factor_secret"),
                            two_factor_recovery_codes: row.get("two_factor_recovery_codes"),
                            two_factor_confirmed_at: timestamp_to_string(row.get("two_factor_confirmed_at")),
                            created_at: timestamptz_to_string(row.get("created_at")),
                            updated_at: timestamptz_to_string(row.get("updated_at")),
                            phone: row.get("phone"),
                            address: row.get("address"),
                            roles: row.get("roles"),
                        }
                    })
                    .fetch_optional(&conn).await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                let access_token = create_access_token()
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error dari access token".to_string()))?;
                let refresh_token = create_refresh_token()
                    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error dari refresh token".to_string()))?;

                sqlx::query("INSERT INTO token_user (user_id, access_token, refresh_token) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET access_token = EXCLUDED.access_token, refresh_token = EXCLUDED.refresh_token;")
                    .bind(user_id)
                    .bind(&access_token)
                    .bind(&refresh_token)
                    .execute(&conn)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                
                let data = ResponseLogin {
                    user: data,
                    token: access_token,
                };

                let result = ResponseModel {
                    status: "Success".to_string(),
                    message: "login successfully".to_string(),
                    data: Some(&data),
                };

                Ok((StatusCode::OK, Json(result)).into_response())
            }
        }
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }
}

pub async fn sign_in(conn: Extension<Pool<Postgres>>, req_sign_in: Json<ReqSignIn>) -> Result<Response, (StatusCode, String)> {
    let conn = conn.0;
    // hash password menggunakan bycrypt
    let hashed_password = bcrypt::hash(req_sign_in.password.clone(), 10)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error from bcrypt".to_string()))?;

    // insert data ke table user
    sqlx::query("INSERT INTO users (name, email, password, phone, roles) VALUES ($1, $2, $3, $4, $5)")
        .bind(&req_sign_in.name)
        .bind(&req_sign_in.email)
        .bind(&hashed_password)
        .bind(&req_sign_in.phone)
        .bind(&req_sign_in.roles)
        .execute(&conn)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // ambil data user yang baru sudah diinsert
    let data = sqlx::query("SELECT name, email, phone, roles, created_at, updated_at, id FROM users WHERE email = $1")
       .bind(&req_sign_in.email)
       .map(|row: PgRow| {
           RegUser {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            phone: row.get("phone"),
            roles: row.get("roles"),
            created_at: timestamptz_to_string(row.get("created_at")),
            updated_at: timestamptz_to_string(row.get("updated_at")),
           }
       })
       .fetch_one(&conn)
       .await
       .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result = ResponseModel {
        status: "Success".to_string(),
        message: "User registered successfully".to_string(),
        data: Some(&data),
    };

    Ok((StatusCode::OK, Json(result)).into_response())
}
