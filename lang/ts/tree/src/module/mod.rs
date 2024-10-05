use crate::{definition::TSDefinition, import::TSImport};

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub doc: Option<String>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
