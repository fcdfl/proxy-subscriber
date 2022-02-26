use askama::Template;

use crate::{args::ArgsMsgOnly, model};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "group/add.html")]
pub struct GroupAdd {}

#[derive(Template)]
#[template(path = "group/index.html")]
pub struct GroupIndex {
    pub args: ArgsMsgOnly,
    pub list: Vec<model::Group>,
}

#[derive(Template)]
#[template(path = "node/add.html")]
pub struct NodeAdd {}
