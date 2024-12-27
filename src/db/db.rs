use deadpool_postgres::Client;
use tokio_postgres::NoTls;
use uuid::Uuid;
use crate::{config::ExampleConfig, models::SecretMessage};
use once_cell::sync::Lazy;
use ::config::Config;
use deadpool_postgres::Pool;

static POOL: Lazy<Pool> = Lazy::new(|| {
    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    config.pg.create_pool(None, NoTls).unwrap()
});

pub fn pool() -> &'static Pool {
    &POOL
}

pub async fn insert_secret_message(client: &Client, message: &str) -> Result<SecretMessage, &'static str> {
    let _stmt = "INSERT INTO secret_message (message) VALUES ($1) RETURNING *";
    let stmt = client.prepare(&_stmt).await.map_err(|_| "Failed to prepare statement")?;

    let rows = client.query(&stmt, &[&message]).await.map_err(|_| "Failed to execute query")?;
    let secret_message = rows.iter().map(|row| SecretMessage::from(row)).collect::<Vec<SecretMessage>>().pop().ok_or("No secret message returned")?;

    Ok(secret_message)
}

pub async fn select_secret_message(client: &Client, id: Uuid) -> Result<SecretMessage, &'static str> {
    let _stmt = "SELECT * FROM secret_message WHERE id = $1";
    let stmt = client.prepare(&_stmt).await.map_err(|_| "Failed to prepare statement")?;

    let rows = client.query(&stmt, &[&id]).await.map_err(|_| "Failed to execute query")?;
    let secret_message = rows.iter().map(|row| SecretMessage::from(row)).collect::<Vec<SecretMessage>>().pop().ok_or("No secret message found")?;

    Ok(secret_message)
}
