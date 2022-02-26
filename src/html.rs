use askama::Template;

use crate::{
    args::{ArgsForNode, ArgsMsgOnly},
    db::Paginate,
    model,
};

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
pub struct NodeAdd {
    pub groups: Vec<model::Group>,
}

#[derive(Template)]
#[template(path = "node/edit.html")]
pub struct NodeEdit {
    pub groups: Vec<model::Group>,
    pub item: model::Node,
}
#[derive(Template)]
#[template(path = "node/index.html")]
pub struct NodeIndex {
    pub args: ArgsForNode,
    pub list: Paginate<Vec<model::Node>>,
}
