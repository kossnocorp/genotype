use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum PyRenderError {
    #[error("Tried to convert unresolved path")]
    #[diagnostic(code(GTPYR101))]
    Todo,
}

impl GtlError for PyRenderError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}
