use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RSModule {
    pub id: GTModuleId,
    pub doc: Option<RSDoc>,
    pub imports: Vec<RSUse>,
    pub definitions: Vec<RSDefinition>,
}
