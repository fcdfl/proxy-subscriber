use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Scheme {
    Trojan,
    Vmess,
    Shadowsocks,
    Socks,
    Http,
}
#[derive(Deserialize, Debug)]
pub struct Node {
    pub name: String,
    pub host: String,
    pub port: u32,
    pub password: Option<String>,
    pub path: Option<String>,
    pub uuid: Option<String>,
    pub alter: Option<u8>,
    pub cipher: Option<String>,
    pub username: Option<String>,
    pub scheme: Scheme,
}
#[derive(Deserialize, Debug)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub url: String,
}
#[derive(Deserialize, Debug)]
pub struct Cfips {
    pub ip: String,
    pub label: String,
}

#[derive(Deserialize, Debug)]
pub struct Auth {
    pub id: i32,
    pub email: String,
    pub password: String,
}
