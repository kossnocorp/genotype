use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RsModule {
    pub id: GtModuleId,
    #[visit]
    pub doc: Option<RsDoc>,
    #[visit]
    pub imports: Vec<RsUse>,
    #[visit]
    pub definitions: Vec<RsDefinition>,
}
