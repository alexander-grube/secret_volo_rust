use ::config::Config;
use deadpool_postgres::Pool;
use tokio_postgres::NoTls;
use uuid::Uuid;
use config::ExampleConfig;
use volo_http::{http::StatusCode, Json, PathParams};
use crate::{config, db::{insert_secret_message, select_secret_message}, models::{NewSecretMessage, SecretMessage}};
use once_cell::sync::Lazy;

static POOL: Lazy<Pool> = Lazy::new(|| {
    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    config.pg.create_pool(None, NoTls).unwrap()
});

fn pool() -> &'static Pool {
    &POOL
}

pub async fn create_secret_message(Json(message): Json<NewSecretMessage>) -> (StatusCode, Result<Json<SecretMessage>, &'static str>) {
    let client = pool().get().await.unwrap();
    match insert_secret_message(&client, &message.message).await {
        Ok(message) => (StatusCode::CREATED, Ok(Json(message))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Err("Failed to create message"))
    }
}

pub async fn get_secret_message(PathParams(id): PathParams<String>) -> (StatusCode, Result<Json<SecretMessage>, &'static str>) {
    let client = pool().get().await.unwrap();
    let id = Uuid::parse_str(&id).unwrap();
    match select_secret_message(&client, id).await {
        Ok(message) => (StatusCode::OK, Ok(Json(message))),
        Err(_) => (StatusCode::NOT_FOUND, Err("Message not found"))
    }
}



