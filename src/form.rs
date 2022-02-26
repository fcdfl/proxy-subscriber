use serde::Deserialize;

use crate::model::Scheme;

#[derive(Debug, Deserialize)]
pub struct GroupCreate {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct NodeCreate {
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
