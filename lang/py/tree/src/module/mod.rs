use crate::{definition::PYDefinition, import::PYImport, PYDoc};

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYModule {
    pub doc: Option<PYDoc>,
    pub imports: Vec<PYImport>,
    pub definitions: Vec<PYDefinition>,
}
