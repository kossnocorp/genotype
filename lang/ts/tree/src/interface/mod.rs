use crate::{identifier::TSIdentifier, property::TSProperty, TSExtension, TSReference};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInterface {
    pub name: TSIdentifier,
    pub extensions: Vec<TSExtension>,
    pub properties: Vec<TSProperty>,
}
