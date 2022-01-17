use askama::Template;
use axum::response::Html;

use crate::html::Index;

pub(crate) async fn index() -> Html<String> {
    let tpl = Index {};
    Html(tpl.render().unwrap())
}
