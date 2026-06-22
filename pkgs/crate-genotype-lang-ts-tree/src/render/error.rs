use crate::prelude::internal::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum TsRenderError {
    #[error("Tried to convert unresolved path")]
    #[diagnostic(code(GTTSR101))]
    Todo,
}

impl GtlError for TsRenderError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}
