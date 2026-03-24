use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSInterface {
    #[visit]
    pub doc: Option<TSDoc>,
    #[visit]
    pub name: TSIdentifier,
    #[visit]
    pub extensions: Vec<TSExtension>,
    #[visit]
    pub properties: Vec<TSProperty>,
}
