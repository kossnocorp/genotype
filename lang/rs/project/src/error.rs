use genotype_parser::GTSpan;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RSProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTRSP101))]
    BuildModulePath(String),

    #[error("Failed to resolve definition extensions")]
    #[diagnostic(code(GTRSP201))]
    FailedExtensionsResolve(#[label("this definition")] GTSpan, String),

    #[error("Only structs can be extended with, but {1} is not a struct")]
    #[diagnostic(code(GTRSP202))]
    NonStructExtension(#[label("this struct extensions")] GTSpan, String),

    #[error("Detected cyclic dependencies")]
    #[diagnostic(code(GTRSP203))]
    CyclicExtensions(#[label(collection, "these structs reference each other")] Vec<GTSpan>),

    #[error("Tuple structs can't be extended with")]
    #[diagnostic(code(GTRSP204))]
    TupleStructExtension(#[label("this struct extensions")] GTSpan),
}
