use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyModule {
    #[visit]
    pub doc: Option<PyDoc>,
    #[visit]
    pub imports: Vec<PyImport>,
    #[visit]
    pub definitions: Vec<PyDefinition>,
}
