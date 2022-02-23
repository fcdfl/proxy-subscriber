use postgres_types::{FromSql, ToSql};
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

#[derive(Deserialize, Debug, Serialize, ToSql, FromSql)]
#[serde(rename_all = "camelCase")]
pub enum Scheme {
    Trojan,
    Vmess,
    Shadowsocks,
    Socks,
    Http,
}
#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "nodes")]
pub struct Node {
    pub id: i32,
    pub name: String,
    pub host: String,
    pub port: u32,
    pub password: Option<String>,
    pub path: Option<String>,
    pub uuid: Option<String>,
    pub alter_id: Option<i8>,
    pub cipher: Option<String>,
    pub username: Option<String>,
    pub scheme: Scheme,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "nodes")]
pub struct NodeID {
    pub id: i32,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "cfips")]
pub struct Cfips {
    pub id: i32,
    pub ip: String,
    pub label: String,
    pub code: String,
}

#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "cfips")]
pub struct CfipsID {
    pub id: i32,
}
#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "auths")]
pub struct Auth {
    pub id: i32,
    pub email: String,
    pub password: String,
}
#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "auths")]
pub struct AuthID {
    pub id: i32,
}
