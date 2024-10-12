use crate::{identifier::TSIdentifier, property::TSProperty, TSReference};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInterface {
    pub name: TSIdentifier,
    pub extensions: Vec<TSReference>,
    pub properties: Vec<TSProperty>,
}
