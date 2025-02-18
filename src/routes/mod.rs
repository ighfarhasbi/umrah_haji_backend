mod home_slider_handler;
mod user_handler;
mod historical_sites_handler;
mod news_handler;
mod doas_handler;
mod faqs_handler;
mod waktu_solat_handler;

use axum::{middleware, routing::{delete, get, post, put}, Extension, Router};
use doas_handler::{add_doas, delete_doas, get_doas, update_doas};
use historical_sites_handler::{add_historical_sites, delete_historical_sites, get_historical_sites, update_historical_sites};
use home_slider_handler::{add_slider, delete_slider, get_sliders, update_slider};
use news_handler::{add_news, delete_news, get_news, update_news};
use faqs_handler::{add_faqs, delete_faqs, get_faqs, update_faqs};
use sqlx::{Pool, Postgres};
use user_handler::{login_user, sign_in};
use waktu_solat_handler::get_prayer_times;

use crate::middleware::guard::guard_route;

pub async fn create_route(conn: Pool<Postgres>) -> Router {
    Router::new()
        // .route("/api/v1/login", get(get_user))
        .route("/api/v1/doas", get(get_doas))
        .route("/api/v1/doas", post(add_doas))
        .route("/api/v1/doas/{id}", put(update_doas))
        .route("/api/v1/doas/{id}", delete(delete_doas))
        .route("/api/v1/news", get(get_news))
        .route("/api/v1/news", post(add_news))
        .route("/api/v1/news/{id}", put(update_news))
        .route("/api/v1/news/{id}", delete(delete_news))
        .route("/api/v1/historical_sites", get(get_historical_sites))
        .route("/api/v1/historical_sites", post(add_historical_sites))
        .route("/api/v1/historical_sites/{id}", put(update_historical_sites))
        .route("/api/v1/historical_sites/{id}", delete(delete_historical_sites))
        .route("/api/v1/slider", get(get_sliders))
        .route("/api/v1/slider", post(add_slider))
        .route("/api/v1/slider/{id}", put(update_slider))
        .route("/api/v1/slider/{id}", delete(delete_slider))
        .route("/api/v1/faqs", get(get_faqs))
        .route("/api/v1/faqs", post(add_faqs))
        .route("/api/v1/faqs/{id}", put(update_faqs))
        .route("/api/v1/faqs/{id}", delete(delete_faqs))
        .route("/api/v1/waktu_solat", get(get_prayer_times))
        .route_layer(middleware::from_fn(guard_route))
        .route("/api/v1/user/sign_in", post(sign_in))
        .route("/api/v1/user/login", post(login_user))
        .route_layer(Extension(conn))
}
