use crate::{error::AppError, Result};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{types::ToSql, GenericClient, Statement};

pub mod topic;
pub mod user;

/// Prepare
async fn get_stmt(client: &impl GenericClient, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

/// Query
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

// CUD
async fn execute(
    client: &impl GenericClient,
    sql: &str,
    params: &[&(dyn Sync + ToSql)],
) -> Result<u64> {
    let stmt = get_stmt(client, sql).await?;
    client.execute(&stmt, params).await.map_err(AppError::from)
}
