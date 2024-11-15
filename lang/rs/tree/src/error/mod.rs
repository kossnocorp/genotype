use genotype_parser::GTSpan;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RSError {
    #[error("Attempted to render unresolved struct fields")]
    #[diagnostic(code(GTRS101))]
    UnresolvedStructFields(#[label("struct converted from this object")] GTSpan),
}
