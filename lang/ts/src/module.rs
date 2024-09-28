use crate::{definition::TSDefinition, import::TSImport};

pub struct TSModule {
    pub path: String,
    pub doc: Option<String>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
