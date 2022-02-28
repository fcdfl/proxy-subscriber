use serde::Serialize;

use crate::model::Node;

pub trait ToClash {
    fn to_clash(&self) -> String;
}

pub trait ToV2ray {
    fn to_v2ray(&self) -> String;
}

pub trait ToSub: ToClash + ToV2ray {}

#[derive(Serialize)]
pub struct WSOptsHeaders {
    pub host: String,
}
#[derive(Serialize)]
pub struct WSOpts {
    pub path: String,
    pub headers: WSOptsHeaders,
}
#[derive(Serialize)]
pub struct VmessNode {
    pub name: String,
    pub types: String,
    pub server: String,
    pub port: i32,
    pub uuid: String,
    pub alter_id: i32,
    pub cipher: String,
    pub tls: bool,
    pub skip_cert_verify: bool,
    pub servername: String,
    pub network: String,
    pub ws_opts: WSOpts,
}

impl From<&Node> for VmessNode {
    fn from(n: &Node) -> Self {
        Self {
            name: n.name.clone(),
            types: "vmess".to_string(),
            server: n.host.clone(),
            port: n.port,
            uuid: n.uuid().clone(),
            alter_id: n.alter_id(),
            cipher: n.cipher().clone(),
            tls: true,
            skip_cert_verify: false,
            servername: n.host.clone(),
            network: "ws".to_string(),
            ws_opts: WSOpts {
                path: n.path().clone(),
                headers: WSOptsHeaders {
                    host: n.host.clone(),
                },
            },
        }
    }
}

#[derive(Serialize)]
pub struct TrojanNode {
    pub name: String,
    pub types: String,
    pub server: String,
    pub port: i32,
    pub password: String,
    pub sni: String,
    pub skip_cert_verify: bool,
    pub alpn: Vec<String>,
    pub network: Option<String>,
    pub ws_opts: Option<WSOpts>,
}

impl From<&Node> for TrojanNode {
    fn from(n: &Node) -> Self {
        Self {
            name: n.name.clone(),
            types: "trojan".to_string(),
            server: n.host.clone(),
            port: n.port,
            password: n.password().clone(),
            sni: n.host.clone(),
            skip_cert_verify: false,
            alpn: vec![],
            network: Some("ws".to_string()),
            ws_opts: Some(WSOpts {
                path: n.path().clone(),
                headers: WSOptsHeaders {
                    host: n.host.clone(),
                },
            }),
        }
    }
}

#[derive(Serialize)]
pub struct SimpleNode {
    pub name: String,
    pub types: String,
    pub server: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}
impl SimpleNode {
    pub fn new(n: &Node, types: &str) -> Self {
        Self {
            types: types.to_string(),
            ..Self::from(n)
        }
    }
    pub fn socks(n: &Node) -> Self {
        Self::new(n, "socks5")
    }
    pub fn http(n: &Node) -> Self {
        Self::new(n, "http")
    }
}
impl From<&Node> for SimpleNode {
    fn from(n: &Node) -> Self {
        Self {
            name: n.name.clone(),
            types: "".to_string(),
            server: n.host.clone(),
            port: n.port,
            username: n.username.clone(),
            password: n.password.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct ShadowsocksNode {
    pub name: String,
    pub types: String,
    pub server: String,
    pub port: i32,
    pub password: String,
    pub cipher: String,
}

impl From<&Node> for ShadowsocksNode {
    fn from(n: &Node) -> Self {
        Self {
            name: n.name.clone(),
            types: "ss".to_string(),
            server: n.host.clone(),
            port: n.port,
            password: n.password().clone(),
            cipher: n.cipher(),
        }
    }
}
