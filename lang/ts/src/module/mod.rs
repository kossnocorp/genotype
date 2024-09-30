use crate::{definition::TSDefinition, import::TSImport};

mod parser;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub doc: Option<String>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
