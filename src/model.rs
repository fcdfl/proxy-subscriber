use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "groups")]
pub struct GroupID {
    pub id: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "groups")]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub is_del: bool,
}
