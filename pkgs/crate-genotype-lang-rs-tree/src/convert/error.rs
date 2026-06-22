use crate::prelude::internal::*;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RsConvertError {
    #[error("Tried to convert unresolved path")]
    #[diagnostic(code(GTRSC101))]
    UnresolvedPath(#[label("this path")] GtSpan),

    #[error("Tried to convert unresolved reference")]
    #[diagnostic(code(GTRSC102))]
    UnresolvedReference(#[label("this reference")] GtSpan),

    #[error("Missing default variant for enum with derived Default")]
    #[diagnostic(code(GTRSC103))]
    MissingDefaultVariant(#[label("enum")] GtSpan),

    #[error("Multiple default variants for enum with derived Default")]
    #[diagnostic(code(GTRSC104))]
    MultipleDefaultVariants(#[label("enum")] GtSpan),
}

impl GtlError for RsConvertError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}
