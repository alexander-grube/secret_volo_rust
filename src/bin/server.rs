use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use dotenv::dotenv;
use tracing_subscriber;

use volo_http::{
    context::ServerContext,
    http::{StatusCode, Uri},
    request::ServerRequest,
    response::ServerResponse,
    server::{
        layer::TimeoutLayer,
        middleware::{self, Next},
        IntoResponse, Router, Server,
    },
    Address,
};
use volo_secret_message::json_test_router;

fn timeout_handler(_: &ServerContext) -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "Timeout!\n")
}

pub async fn trace_request(
    peer: Address,
    uri: Uri,
    cx: &mut ServerContext,
    req: ServerRequest,
    next: Next,
) -> ServerResponse {
    let start = Instant::now();
    let ret = next.run(cx, req).await.into_response();
    let status = ret.status();
    let cost = Instant::now().duration_since(start);
    tracing::info!("`{peer}` request `{uri}`, response {status}, cost {cost:?}");
    ret
}

#[volo::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .merge(json_test_router())
        .layer(middleware::from_fn(trace_request))
        .layer(TimeoutLayer::new(Duration::from_secs(1), timeout_handler));

    let addr = "[::]:8080".parse::<SocketAddr>().unwrap();
    let addr = Address::from(addr);

    println!("Listening on {addr}");

    Server::new(app).run(addr).await.unwrap();
}
