use chrono::{TimeZone, Utc};
use serde::Serialize;
use std::time;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "users")]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "topics")]
pub struct TopicContent {
    pub html: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "topics")]
pub struct TopicArchive {
    pub dateline: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "topics")]
pub struct TopicList {
    pub id: i64,
    pub title: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "topics")]
pub struct TopicEdit {
    pub title: String,
    pub summary: String,
    pub markdown: String,
}

#[derive(PostgresMapper, Serialize)]
#[pg_mapper(table = "topics")]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub dateline: time::SystemTime,
}

impl Topic {
    pub fn dateline(&self) -> String {
        dateline(self.dateline)
    }
}

fn dateline(dt: time::SystemTime) -> String {
    let secs = dt.duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i64;
    Utc.timestamp_opt(secs, 0)
        .unwrap()
        .format("%Y/%m/%d %H:%M:%S Utc")
        .to_string()
}
