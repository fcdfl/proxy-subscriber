use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{
    types::{FromSqlOwned, ToSql},
    GenericClient, Statement,
};

use crate::{error::AppError, Result};

pub mod group;

type ParamterItem = (dyn ToSql + Sync);
type Paramters<'a> = &'a [&'a ParamterItem];

async fn get_stmt(client: &impl GenericClient, sql: &str) -> Result<Statement> {
    client.prepare(sql).await.map_err(AppError::from)
}

async fn query<'a, C, T>(client: &C, sql: &str, params: Paramters<'a>) -> Result<Vec<T>>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    let stmt = get_stmt(client, sql).await?;
    let r = client
        .query(&stmt, params)
        .await
        .map_err(AppError::from)?
        .iter()
        .map(|row| <T>::from_row_ref(row).unwrap())
        .collect::<Vec<T>>();
    Ok(r)
}

async fn query_row_opt_msg<'a, C, T>(
    client: &C,
    sql: &str,
    params: Paramters<'a>,
    msg: Option<&str>,
) -> Result<T>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    Ok(query(client, sql, params)
        .await?
        .pop()
        .ok_or(AppError::not_found_msg_opt(msg))?)
}
async fn query_row<'a, C, T>(client: &C, sql: &str, params: Paramters<'a>) -> Result<T>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    query_row_opt_msg(client, sql, params, None).await
}
async fn insert<'a, C, T>(client: &C, sql: &str, params: Paramters<'a>, msg: &str) -> Result<T>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    query_row_opt_msg(client, sql, params, Some(msg)).await
}
async fn query_col<'a, C, T>(client: &C, sql: &str, params: Paramters<'a>) -> Result<T>
where
    C: GenericClient,
    T: FromSqlOwned,
{
    let stmt = get_stmt(client, sql).await?;
    client
        .query_one(&stmt, params)
        .await
        .map_err(AppError::from)?
        .try_get(0)
        .map_err(AppError::from)
}
async fn execute<'a, C>(client: &C, sql: &str, params: Paramters<'a>) -> Result<u64>
where
    C: GenericClient,
{
    let stmt = get_stmt(client, sql).await?;
    client.execute(&stmt, params).await.map_err(AppError::from)
}
async fn del_or_restore<'a, C>(
    client: &C,
    table: &str,
    is_del: bool,
    id: &ParamterItem,
) -> Result<u64>
where
    C: GenericClient,
{
    let sql = format!("UPDATE {} SET is_del=$1 WHERE id=$2", table);
    let stmt = get_stmt(client, &sql).await?;
    let params = &[&is_del, id];
    client.execute(&stmt, params).await.map_err(AppError::from)
}
async fn count<'a, C>(client: &C, sql: &str, params: Paramters<'a>) -> Result<u64>
where
    C: GenericClient,
{
    let r: i64 = query_col(client, sql, params).await?;
    Ok(r as u64)
}
