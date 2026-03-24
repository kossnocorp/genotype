use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYModule {
    #[visit]
    pub doc: Option<PYDoc>,
    #[visit]
    pub imports: Vec<PYImport>,
    #[visit]
    pub definitions: Vec<PYDefinition>,
}
