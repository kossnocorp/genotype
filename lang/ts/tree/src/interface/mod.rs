use crate::{identifier::TSIdentifier, property::TSProperty, TSDoc, TSExtension};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSInterface {
    pub doc: Option<TSDoc>,
    pub name: TSIdentifier,
    pub extensions: Vec<TSExtension>,
    pub properties: Vec<TSProperty>,
}
