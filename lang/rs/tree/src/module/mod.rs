use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct RSModule {
    pub id: GTModuleId,
    #[visit]
    pub doc: Option<RSDoc>,
    #[visit]
    pub imports: Vec<RSUse>,
    #[visit]
    pub definitions: Vec<RSDefinition>,
}
