use crate::prelude::internal::*;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RsRenderError {
    #[error("Attempted to render unresolved struct fields")]
    #[diagnostic(code(GTRSR101))]
    UnresolvedStructFields(#[label("struct converted from this object")] GtSpan),
}

impl GtlError for RsRenderError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}
