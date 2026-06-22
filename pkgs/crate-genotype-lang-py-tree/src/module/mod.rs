use crate::prelude::internal::*;

mod convert;

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

impl GtlModule<'_> for PyModule {
    type Import = PyImport;
    type RenderTypes = PyRenderTypes;

    fn imports(&self) -> Vec<&PyImport> {
        self.imports.iter().collect()
    }
}
