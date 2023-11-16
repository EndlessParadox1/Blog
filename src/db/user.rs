use crate::{model::User, Result};
use tokio_postgres::Client;

pub async fn find(client: &Client, username: &str) -> Result<Vec<User>> {
    super::query(
        client,
        "SELECT * FROM users WHERE username = $1",
        &[&username],
    )
    .await
}

pub async fn create(client: &Client, username: &str, password: &str) -> Result<()> {
    super::execute(
        client,
        "INSERT INTO users VALUES ($1, $2)",
        &[&username, &password],
    )
    .await?;
    Ok(())
}
