use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "group/add.html")]
pub struct GroupAdd {}

#[derive(Template)]
#[template(path = "node/add.html")]
pub struct NodeAdd {}
