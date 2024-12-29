mod handlers;
mod db;
mod config;
mod models;
mod auth;

use volo_http::{context::ServerContext, http::StatusCode, request::ServerRequest, response::ServerResponse, server::{middleware::Next, route::{get, post, Router}, IntoResponse}};

pub fn secret_router() -> Router {
    Router::new()
        .route("/volo/secret", post(handlers::create_secret_message))
        .route("/volo/secret/{id}", get(handlers::get_secret_message))
        .route("/volo/token", get(handlers::create_jwt_token))
}

pub async fn jwt_middleware(
    cx: &mut ServerContext,
    req: ServerRequest,
    next: Next,
) -> ServerResponse {
    if req.uri().path() == "/volo/token" {
        return next.run(cx, req).await.into_response();
    }
    let token = match req.headers().get("Authorization") {
        Some(token) => {
            let token = token.to_str().unwrap();
            if token.starts_with("Bearer ") {
                Some(token[7..].to_string())
            } else {
                None
            }
        }
        None => return (StatusCode::UNAUTHORIZED, "No token found").into_response(),
    };

    if !auth::verify_jwt_token(token.expect("Token should be present")).unwrap() {
        return (StatusCode::UNAUTHORIZED, "Invalid token").into_response();
    }

    next.run(cx, req).await.into_response()
}