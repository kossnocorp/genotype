use crate::prelude::internal::*;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum GtfmtError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parse(#[from] GtParseError),
}
