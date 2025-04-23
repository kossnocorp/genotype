use crate::prelude::internal::*;

pub mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct PYModule {
    pub doc: Option<PYDoc>,
    pub imports: Vec<PYImport>,
    pub definitions: Vec<PYDefinition>,
}
