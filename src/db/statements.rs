pub const INSERT_SECRET_MESSAGE: &str = "INSERT INTO secret_message (message) VALUES ($1) RETURNING *";
pub const SELECT_SECRET_MESSAGE: &str = "SELECT * FROM secret_message WHERE id = $1";