use crate::{identifier::TSIdentifier, property::TSProperty};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInterface {
    pub name: TSIdentifier,
    pub properties: Vec<TSProperty>,
}
