use crate::prelude::internal::*;

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub doc: Option<TSDoc>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
