use crate::{definition::TSDefinition, import::TSImport, TSPath};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub path: TSPath,
    pub doc: Option<String>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
