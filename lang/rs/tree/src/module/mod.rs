use crate::{definition::RSDefinition, import::RSImport, RSDoc};

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct RSModule {
    pub doc: Option<RSDoc>,
    pub imports: Vec<RSImport>,
    pub definitions: Vec<RSDefinition>,
}
