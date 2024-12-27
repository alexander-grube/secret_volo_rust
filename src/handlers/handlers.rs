use uuid::Uuid;
use volo_http::{http::StatusCode, Json, PathParams};
use crate::{db::{insert_secret_message, select_secret_message, pool}, models::{NewSecretMessage, SecretMessage}};

pub async fn create_secret_message(Json(message): Json<NewSecretMessage>) -> (StatusCode, Result<Json<SecretMessage>, &'static str>) {
    let client = pool().get().await.unwrap();
    match insert_secret_message(&client, &message.message).await {
        Ok(message) => (StatusCode::CREATED, Ok(Json(message))),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, Err(error))
    }
}

pub async fn get_secret_message(PathParams(id): PathParams<String>) -> (StatusCode, Result<Json<SecretMessage>, &'static str>) {
    let client = pool().get().await.unwrap();
    let id = Uuid::parse_str(&id).unwrap();
    match select_secret_message(&client, id).await {
        Ok(message) => (StatusCode::OK, Ok(Json(message))),
        Err(error) => (StatusCode::NOT_FOUND, Err(error))
    }
}
