mod home_slider_handler;
mod user_handler;
mod historical_sites_handler;
mod news_handler;
mod doas_handler;

use axum::{middleware, routing::{get, post}, Extension, Router};
use doas_handler::get_doas;
use historical_sites_handler::get_historical_sites;
use home_slider_handler::get_sliders;
use news_handler::get_news;
use sqlx::{Pool, Postgres};
use user_handler::{login_user, sign_in};

use crate::middleware::guard::guard_route;

pub async fn create_route(conn: Pool<Postgres>) -> Router {
    Router::new()
        // .route("/api/v1/login", get(get_user))
        .route("/api/v1/doas", get(get_doas))
        .route("/api/v1/news", get(get_news))
        .route("/api/v1/historical_sites", get(get_historical_sites))
        .route("/api/v1/slider", get(get_sliders))
        .route_layer(middleware::from_fn(guard_route))
        .route("/api/v1/user/sign_in", post(sign_in))
        .route("/api/v1/user/login", post(login_user))
        .route_layer(Extension(conn))
}
