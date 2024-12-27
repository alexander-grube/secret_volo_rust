use deadpool_postgres::Client;
use uuid::Uuid;
use crate::models::SecretMessage;

pub async fn insert_secret_message(client: &Client, message: &str) -> Result<SecretMessage, Box<dyn std::error::Error>> {
    let _stmt = "INSERT INTO secret_message (message) VALUES ($1) RETURNING *";
    let stmt = client.prepare(&_stmt).await?;

    let rows = client.query(&stmt, &[&message]).await?;
    let secret_message = rows.iter().map(|row| SecretMessage::from(row)).collect::<Vec<SecretMessage>>().pop().ok_or("No secret message returned")?;

    Ok(secret_message)
}

pub async fn select_secret_message(client: &Client, id: Uuid) -> Result<SecretMessage, Box<dyn std::error::Error>> {
    let _stmt = "SELECT * FROM secret_message WHERE id = $1";
    let stmt = client.prepare(&_stmt).await?;

    let rows = client.query(&stmt, &[&id]).await?;
    let secret_message = rows.iter().map(|row| SecretMessage::from(row)).collect::<Vec<SecretMessage>>().pop().ok_or("No secret message found")?;

    Ok(secret_message)
}
