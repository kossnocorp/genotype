use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;
pub use render::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RSModule {
    pub id: GTModuleId,
    pub doc: Option<RSDoc>,
    pub imports: Vec<RSUse>,
    pub definitions: Vec<RSDefinition>,
}
