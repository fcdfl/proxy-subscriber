use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Extension, Form, Path, Query},
    http::{header, HeaderMap, StatusCode},
    response::Html,
};
use deadpool_postgres::Client;

use crate::{
    args::{ArgsForNode, ArgsForSub, ArgsMsgOnly},
    db::{group, node},
    error::AppError,
    form,
    html::{GroupAdd, GroupIndex, Index, NodeAdd, NodeEdit, NodeIndex},
    model,
    types::{ShadowsocksNode, SimpleNode, TrojanNode, VmessNode},
    Result,
};

type HtmlResponse = Html<String>;
type HtmlResponseResult = Result<HtmlResponse>;
type RedirectResult = Result<(StatusCode, HeaderMap, ())>;

async fn get_client(state: &model::AppState) -> Result<Client> {
    state.pool.get().await.map_err(AppError::from)
}

fn log_error(handler: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler = handler.to_string();
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler, err);
        err
    })
}
fn render(tpl: impl Template) -> HtmlResponseResult {
    let html = tpl.render().map_err(AppError::from)?;
    Ok(Html(html))
}
fn redirect_with_msg_opt(url: &str, msg: Option<&str>) -> RedirectResult {
    let mut headers = HeaderMap::new();
    let url = match msg {
        Some(msg) => format!("{}?msg={}", url, msg),
        None => String::from(url),
    };
    headers.insert(header::LOCATION, (&url).parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
}
fn redirect_with_msg(url: &str, msg: &str) -> RedirectResult {
    redirect_with_msg_opt(url, Some(msg))
}
fn redirect(url: &str) -> RedirectResult {
    redirect_with_msg_opt(url, None)
}

pub(crate) async fn index() -> HtmlResponseResult {
    let handler = "index";
    let tpl = Index {};
    render(tpl).map_err(log_error(handler))
}

pub(crate) async fn group_add_ui() -> HtmlResponseResult {
    let handler = "group_add_ui";
    let tpl = GroupAdd {};
    render(tpl).map_err(log_error(handler))
}
pub(crate) async fn group_index(
    Extension(state): Extension<Arc<model::AppState>>,
    Query(args): Query<ArgsMsgOnly>,
) -> HtmlResponseResult {
    let handler = "group_index";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    let groups = group::all(&client).await.map_err(log_error(handler))?;
    let tpl = GroupIndex { args, list: groups };
    render(tpl).map_err(log_error(handler))
}
pub(crate) async fn group_add(
    Extension(state): Extension<Arc<model::AppState>>,
    Form(frm): Form<form::GroupCreate>,
) -> RedirectResult {
    let handler = "group_add";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    group::create(&client, frm.name)
        .await
        .map_err(log_error(handler))?;
    redirect_with_msg("/admin/group", "分组添加成功")
}

pub(crate) async fn group_del(
    Extension(state): Extension<Arc<model::AppState>>,
    Path(id): Path<i32>,
) -> RedirectResult {
    let handler = "group_del";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    group::del_or_restore(&client, id, true)
        .await
        .map_err(log_error(handler))?;
    redirect_with_msg("/admin/group", "分组删除成功")
}
pub(crate) async fn node_add_ui(
    Extension(state): Extension<Arc<model::AppState>>,
) -> HtmlResponseResult {
    let handler = "node_add_ui";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    let groups = group::all(&client).await.map_err(log_error(handler))?;
    let tpl = NodeAdd { groups };
    render(tpl).map_err(log_error(handler))
}
pub(crate) async fn node_add(
    Extension(state): Extension<Arc<model::AppState>>,
    Form(frm): Form<form::NodeCreate>,
) -> RedirectResult {
    let handler = "node_add";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    node::create(&client, frm)
        .await
        .map_err(log_error(handler))?;
    redirect_with_msg("/admin/node", "节点添加成功")
}
pub(crate) async fn node_index(
    Extension(state): Extension<Arc<model::AppState>>,
    Query(args): Query<ArgsForNode>,
) -> HtmlResponseResult {
    let handler = "node_index";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    let list = node::list(&client, args.page())
        .await
        .map_err(log_error(handler))?;
    let tpl = NodeIndex { args, list };
    render(tpl).map_err(log_error(handler))
}
pub(crate) async fn node_edit_ui(
    Extension(state): Extension<Arc<model::AppState>>,
    Path(id): Path<i32>,
) -> HtmlResponseResult {
    let handler = "node_edit_ui";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    let groups = group::all(&client).await.map_err(log_error(handler))?;
    let item = node::find(&client, id).await.map_err(log_error(handler))?;
    let tpl = NodeEdit { groups, item };
    render(tpl).map_err(log_error(handler))
}
pub(crate) async fn node_edit(
    Extension(state): Extension<Arc<model::AppState>>,
    Form(frm): Form<form::NodeCreate>,
    Path(id): Path<i32>,
) -> RedirectResult {
    let handler = "node_edit";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    node::update(&client, id, frm)
        .await
        .map_err(log_error(handler))?;
    redirect_with_msg("/admin/node", "节点修改成功")
}
pub(crate) async fn node_del(
    Extension(state): Extension<Arc<model::AppState>>,
    Path(id): Path<i32>,
) -> RedirectResult {
    let handler = "node_del";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    node::del_or_restore(&client, id, true)
        .await
        .map_err(log_error(handler))?;
    redirect_with_msg("/admin/node", "节点删除成功")
}
pub(crate) async fn subscriber(
    Extension(state): Extension<Arc<model::AppState>>,
    Path(uuid): Path<String>,
    Query(args): Query<ArgsForSub>,
) -> Result<(HeaderMap, String)> {
    let handler = "subscriber";
    let client = get_client(&state).await.map_err(log_error(handler))?;
    let list = node::subscriber(&client, uuid)
        .await
        .map_err(log_error(handler))?;
    let mut hm = HeaderMap::new();
    if args.c == "clash" {
        let mut output = vec![];
        list.iter().for_each(|n| match n.scheme {
            model::Scheme::Trojan => {
                output.push(serde_yaml::to_value(&TrojanNode::from(n)).unwrap())
            }
            model::Scheme::Vmess => output.push(serde_yaml::to_value(&VmessNode::from(n)).unwrap()),
            model::Scheme::Shadowsocks => {
                output.push(serde_yaml::to_value(&ShadowsocksNode::from(n)).unwrap())
            }
            model::Scheme::Socks => {
                output.push(serde_yaml::to_value(&SimpleNode::socks(n)).unwrap())
            }
            model::Scheme::Http => output.push(serde_yaml::to_value(&SimpleNode::http(n)).unwrap()),
        });
        hm.insert(header::CONTENT_TYPE, "application/yaml".parse().unwrap());
        return Ok((hm, serde_yaml::to_string(&output).unwrap()));
    }
    if args.c == "v2ray" {
        hm.insert(
            header::CONTENT_TYPE,
            "application/json;charset=utf-8".parse().unwrap(),
        );
        return Ok((hm, "".to_string()));
    }
    Err(AppError::invalid_param_msg("invalid request"))
}
