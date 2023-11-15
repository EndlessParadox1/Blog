use crate::{model::User, Result};
use tokio_postgres::Client;

pub async fn find(client: &Client, username: &str) -> Result<User> {
    super::query_row(client, "SELECT * FROM users WHERE email = $1", &[&username]).await
}

pub async fn create(client: &Client, username: &str, password: &str) -> Result<u64> {
    super::execute(
        client,
        "INSERT INTO users VALUES ($1, $2)",
        &[&username, &password],
    )
    .await
}
