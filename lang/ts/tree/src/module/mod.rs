use crate::{definition::TSDefinition, import::TSImport, TSDoc};

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub doc: Option<TSDoc>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
