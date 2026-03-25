use crate::prelude::internal::*;
use miette::Diagnostic;
use thiserror::Error;

mod convert;
pub use convert::*;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RsError {
    #[error("Attempted to render unresolved struct fields")]
    #[diagnostic(code(GTRS101))]
    UnresolvedStructFields(#[label("struct converted from this object")] GtSpan),
}
