use crate::prelude::internal::*;

#[derive(thiserror::Error, Diagnostic, Debug, PartialEq, Clone)]
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

    #[error("Newtypes structs can't be extended with")]
    #[diagnostic(code(GTRSP204))]
    TupleStructExtension(#[label("this struct extensions")] GTSpan),

    #[error("Unit structs can't be extended with")]
    #[diagnostic(code(GTRSP205))]
    UnitStructExtension(#[label("this struct extensions")] GTSpan),

    #[error("Failed to parse the [rust.package]")]
    #[diagnostic(code(GTRSP301))]
    ParsePackage(#[from] toml_edit::TomlError),

    #[error("Failed to edit the Cargo.toml content: {0}")]
    #[diagnostic(code(GTRSP302))]
    EditCargo(String),
}
