use askama::Template;
use axum::{
    extract::Form,
    http::{header, HeaderMap, StatusCode},
    response::Html,
};

use crate::{
    error::AppError,
    form,
    html::{GroupAdd, Index, NodeAdd},
    Result,
};

type HtmlResponse = Html<String>;
type HtmlResponseResult = Result<HtmlResponse>;
type RedirectResult = Result<(StatusCode, HeaderMap, ())>;

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
pub(crate) async fn group_add(Form(frm): Form<form::GroupCreate>) -> RedirectResult {
    tracing::debug!("{:?}", frm);
    redirect_with_msg("/admin/group", "分组添加成功")
}

pub(crate) async fn node_add_ui() -> HtmlResponseResult {
    let handler = "node_add_ui";
    let tpl = NodeAdd {};
    render(tpl).map_err(log_error(handler))
}
pub(crate) async fn node_add(Form(frm): Form<form::NodeCreate>) -> RedirectResult {
    tracing::debug!("{:?}", frm);
    redirect_with_msg("/admin/node", "节点添加成功")
}
