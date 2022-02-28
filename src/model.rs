use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::types::{ToClash, ToV2ray};

pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}

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
#[serde(rename_all = "lowercase")]
#[postgres(name = "schemes")]
pub enum Scheme {
    #[postgres(name = "trojan")]
    Trojan,
    #[postgres(name = "vmess")]
    Vmess,
    #[postgres(name = "shadowsocks")]
    Shadowsocks,
    #[postgres(name = "socks")]
    Socks,
    #[postgres(name = "http")]
    Http,
}
impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(PostgresMapper, Serialize, Deserialize)]
#[pg_mapper(table = "nodes")]
pub struct Node {
    pub id: i32,
    pub group_id: i32,
    pub name: String,
    pub host: String,
    pub port: i32,
    pub password: Option<String>,
    pub path: Option<String>,
    pub uuid: Option<String>,
    pub alter_id: Option<i32>,
    pub cipher: Option<String>,
    pub username: Option<String>,
    pub scheme: Scheme,
}

impl Node {
    pub fn password(&self) -> String {
        match &self.password {
            Some(s) => s.to_string(),
            None => "".to_string(),
        }
    }
    pub fn path(&self) -> String {
        match &self.path {
            Some(s) => s.to_string(),
            None => "".to_string(),
        }
    }
    pub fn uuid(&self) -> String {
        match &self.uuid {
            Some(s) => s.to_string(),
            None => "".to_string(),
        }
    }
    pub fn alter_id(&self) -> i32 {
        match &self.alter_id {
            Some(i) => *i,
            None => 0,
        }
    }
    pub fn cipher(&self) -> String {
        match &self.cipher {
            Some(s) => s.to_string(),
            None => "".to_string(),
        }
    }
    pub fn username(&self) -> String {
        match &self.username {
            Some(s) => s.to_string(),
            None => "".to_string(),
        }
    }
}
impl ToClash for Node {
    fn to_clash(&self) -> String {
        unreachable!()
    }
}
impl ToV2ray for Node {
    fn to_v2ray(&self) -> String {
        let url = match self.scheme {
            Scheme::Vmess => {
                let u = format!("");
                u
            }
            Scheme::Trojan => {
                format!(
                    "{password}@{host}:{port}?sni={host}#{name}",
                    password = self.password(),
                    host = self.host,
                    name = self.name, // url encode
                    port = self.port,
                )
            }
            Scheme::Socks => {
                format!("{host}:{port}", host = self.host, port = self.port)
            }
            _ => "".to_string(),
        };
        format!("{}://{}", self.scheme, url)
    }
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
