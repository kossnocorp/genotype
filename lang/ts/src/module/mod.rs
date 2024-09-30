use crate::{definition::TSDefinition, import::TSImport};

mod from;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub path: String,
    pub doc: Option<String>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
