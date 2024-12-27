mod handlers;
mod db;
mod config;
mod models;

use volo_http::server::route::{get, post, Router};


pub fn json_test_router() -> Router {
    Router::new()
        .route("/volo/secret", post(handlers::create_secret_message))
        .route("/volo/secret/{id}", get(handlers::get_secret_message))
}