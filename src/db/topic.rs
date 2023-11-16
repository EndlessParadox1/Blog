use crate::{form::Topic, md::to_html, model, Result};
use std::time;
use tokio_postgres::Client;

pub async fn create(client: &Client, frm: &Topic, username: String) -> Result<bool> {
    let html = to_html(&frm.markdown);
    let dateline = time::SystemTime::now();
    let sql = "INSERT INTO topics (title, summary, html, markdown, dateline, writer) VALUES ($1, $2, $3, $4, $5, $6)";
    let n = super::execute(
        client,
        sql,
        &[
            &frm.title,
            &frm.summary,
            &html,
            &frm.markdown,
            &dateline,
            &username,
        ],
    )
    .await?;
    Ok(n > 0)
}

pub async fn del(client: &Client, id: i64) -> Result<bool> {
    let n = super::del(client, "topics", &id).await?;
    Ok(n > 0)
}

pub async fn list_latest(client: &Client, username: String) -> Result<Vec<model::Topic>> {
    let sql = "SELECT id, title, summary, dateline from topics WHERE writer = $1 ORDER BY dateline DESC LIMIT 10";
    super::query(client, sql, &[&username]).await
}

pub async fn list_arch(client: &Client, username: String, dt: String) -> Result<Vec<model::Topic>> {
    let sql =
        "SELECT id, title, summary, dateline from topics WHERE writer = $1";
    let sql = format!("{} AND dateline BETWEEN '{}'::timestamp AND '{}'::timestamp + (INTERVAL '1' MONTH) - (INTERVAL '1' SECOND) ORDER BY dateline DESC", sql, &dt, &dt);
    super::query(client, &sql, &[&username]).await
}

pub async fn archive(client: &Client, username: String) -> Result<Vec<model::TopicArchive>> {
    let sql = "SELECT DISTINCT to_char(DATE_TRUNC('month', dateline), 'YYYY-MM') AS dateline FROM topics WHERE writer = $1";
    super::query(client, sql, &[&username]).await
}

pub async fn list_all(client: &Client, username: String) -> Result<Vec<model::TopicList>> {
    let sql = "SELECT id, title from topics WHERE writer = $1 ORDER BY dateline DESC";
    super::query(client, sql, &[&username]).await
}

pub async fn update(client: &Client, frm: &Topic, id: i64) -> Result<()> {
    let html = to_html(&frm.markdown);
    let dateline = time::SystemTime::now();
    let sql = "UPDATE topics SET title = $1, summary = $2, html = $3, markdown = $4, dateline = $5 WHERE id = $6";
    super::execute(
        client,
        sql,
        &[
            &frm.title,
            &frm.summary,
            &html,
            &frm.markdown,
            &dateline,
            &id,
        ],
    )
    .await?;
    Ok(())
}

pub async fn content(client: &Client, id: i64) -> Result<model::TopicContent> {
    let sql = "SELECT html from topics WHERE id = $1";
    super::query_row_msg(client, sql, &[&id], "Data required non-existent!").await
}

pub async fn edit_data(client: &Client, id: i64) -> Result<model::TopicEdit> {
    let sql = "SELECT title, summary, markdown from topics WHERE id = $1";
    super::query_row_msg(client, sql, &[&id], "Data required non-existent!").await
}
