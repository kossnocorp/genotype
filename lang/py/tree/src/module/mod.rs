use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct PYModule {
    pub doc: Option<PYDoc>,
    pub imports: Vec<PYImport>,
    pub definitions: Vec<PYDefinition>,
}
