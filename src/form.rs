use serde::Deserialize;

use crate::types::Node;

pub type NodeCreate = Node;

#[derive(Debug, Deserialize)]
pub struct GroupCreate {
    pub name: String,
}
