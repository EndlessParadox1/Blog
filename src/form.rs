use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Topic {
    pub title: String,
    pub summary: String,
    pub markdown: String,
}
