use crate::{definition::PYDefinition, import::PYImport};

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYModule {
    pub doc: Option<String>,
    pub imports: Vec<PYImport>,
    pub definitions: Vec<PYDefinition>,
}
