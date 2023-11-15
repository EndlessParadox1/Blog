use crate::{error::AppError, Result};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{types::ToSql, GenericClient, Statement};

pub mod topic;
pub mod user;

/// Prepare
async fn get_stmt(client: &impl GenericClient, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

/// Query multi row
async fn query<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn Sync + ToSql)],
) -> Result<Vec<T>>
where
    T: FromTokioPostgresRow,
{
    let stmt = get_stmt(client, sql).await?;
    let result = client
        .query(&stmt, params)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| <T>::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(result)
}

/// Query single row
async fn query_row_opt<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn Sync + ToSql)],
    msg: Option<String>,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query(client, sql, params)
        .await?
        .pop()
        .ok_or(AppError::notfound_opt(msg))
}

async fn query_row_msg<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn Sync + ToSql)],
    msg: &str,
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, Some(msg.to_string())).await
}

async fn query_row<T>(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn Sync + ToSql)],
) -> Result<T>
where
    T: FromTokioPostgresRow,
{
    query_row_opt(client, sql, params, None).await
}

async fn execute(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn Sync + ToSql)],
) -> Result<u64> {
    let stmt = get_stmt(client, sql).await?;
    client.execute(&stmt, params).await.map_err(AppError::from)
}

async fn del(client: &impl GenericClient, table: &str, id: &(dyn Sync + ToSql)) -> Result<u64> {
    let sql = format!("UPDATE {} SET is_del = true, title = null, summary = null, markdown = null, html = null, dateline = null WHERE id = $2", table);
    execute(client, &sql, &[id]).await
}
