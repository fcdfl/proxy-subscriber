use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{
    types::{FromSqlOwned, ToSql},
    GenericClient, Statement,
};

use crate::{error::AppError, Result};

pub mod cfip;
pub mod group;
pub mod node;

const DEFAULT_PAGE_SIZE: u8 = 30;
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

async fn paginate<'a, C, T>(
    client: &C,
    sql: &str,
    count_sql: &str,
    params: Paramters<'a>,
    page: u32,
) -> Result<Paginate<Vec<T>>>
where
    C: GenericClient,
    T: FromTokioPostgresRow,
{
    let total_record = count::<C>(client, count_sql, params).await?;
    let list = query::<C, T>(client, sql, params).await?;
    Ok(Paginate::new(total_record, page, list))
}

pub struct Paginate<T> {
    pub total_record: u64,
    pub page: u32,
    pub page_size: u8,
    pub total_page: u64,
    pub data: T,
}

impl<T> Paginate<T> {
    pub fn new_with_page_size(total_record: u64, page: u32, page_size: u8, data: T) -> Self {
        let total_page = f64::ceil(total_record as f64 / page_size as f64) as u64;
        Self {
            total_record,
            page,
            page_size,
            total_page,
            data,
        }
    }
    pub fn new(total_record: u64, page: u32, data: T) -> Self {
        Self::new_with_page_size(total_record, page, DEFAULT_PAGE_SIZE, data)
    }
    pub fn has_prev(&self) -> bool {
        self.page > 0
    }
    pub fn has_next(&self) -> bool {
        (self.page as u64) < (self.total_page - 1)
    }
    pub fn is_active(&self, i: &u64) -> bool {
        (*i) == (self.page as u64)
    }
}
