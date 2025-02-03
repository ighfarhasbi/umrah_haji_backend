mod slider_handler;
mod user_handler;

use axum::{middleware, routing::{get, post}, Extension, Router};
use slider_handler::get_sliders;
use sqlx::{Pool, Postgres};
use user_handler::{login_user, sign_in};

use crate::middleware::guard::guard_route;

pub async fn create_route(conn: Pool<Postgres>) -> Router {
    Router::new()
        // .route("/api/v1/login", get(get_user))
        .route("/api/v1/slider", get(get_sliders))
        .route_layer(middleware::from_fn(guard_route))
        .route("/api/v1/user/sign_in", post(sign_in))
        .route("/api/v1/user/login", post(login_user))
        .route_layer(Extension(conn))
}
