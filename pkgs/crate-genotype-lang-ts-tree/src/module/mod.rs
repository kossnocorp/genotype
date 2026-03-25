use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsModule {
    #[visit]
    pub doc: Option<TsDoc>,
    #[visit]
    pub imports: Vec<TsImport>,
    #[visit]
    pub definitions: Vec<TsDefinition>,
}
