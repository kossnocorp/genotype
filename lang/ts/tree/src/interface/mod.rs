use crate::{name::TSName, property::TSProperty};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInterface {
    pub name: TSName,
    pub properties: Vec<TSProperty>,
}
