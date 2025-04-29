use crate::prelude::internal::*;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RSConverterError {
    #[error("Tried to convert unresolved path")]
    #[diagnostic(code(GTRSC101))]
    UnresolvedPath(#[label("this path")] GTSpan),
    #[error("Tried to convert unresolved reference")]
    #[diagnostic(code(GTRSC102))]
    UnresolvedReference(#[label("this reference")] GTSpan),
}
