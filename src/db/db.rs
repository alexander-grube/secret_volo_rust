use std::sync::OnceLock;

use deadpool_postgres::Client;
use tokio_postgres::NoTls;
use uuid::Uuid;
use crate::{config::ExampleConfig, models::SecretMessage};
use ::config::Config;
use deadpool_postgres::Pool;

use super::statements;

static POOL: OnceLock<Pool> = OnceLock::new();

pub fn pool() -> &'static Pool {
    POOL.get_or_init(|| {
        let config_ = Config::builder()
            .add_source(::config::Environment::default())
            .build()
            .unwrap();

        let config: ExampleConfig = config_.try_deserialize().unwrap();

        config.pg.create_pool(None, NoTls).unwrap()
    })
}

pub async fn insert_secret_message(client: &Client, message: &str) -> Result<SecretMessage, &'static str> {
    let _stmt = statements::INSERT_SECRET_MESSAGE;
    let stmt = client.prepare(&_stmt).await.map_err(|_| "Failed to prepare statement")?;

    let rows = client.query(&stmt, &[&message]).await.map_err(|_| "Failed to execute query")?;
    let secret_message = rows.iter().map(|row| SecretMessage::from(row)).collect::<Vec<SecretMessage>>().pop().ok_or("No secret message returned")?;

    Ok(secret_message)
}

pub async fn select_secret_message(client: &Client, id: Uuid) -> Result<SecretMessage, &'static str> {
    let _stmt = statements::SELECT_SECRET_MESSAGE;
    let stmt = client.prepare(&_stmt).await.map_err(|_| "Failed to prepare statement")?;

    let rows = client.query(&stmt, &[&id]).await.map_err(|_| "Failed to execute query")?;
    let secret_message = rows.iter().map(|row| SecretMessage::from(row)).collect::<Vec<SecretMessage>>().pop().ok_or("No secret message found")?;

    Ok(secret_message)
}
