use crate::prelude::internal::*;

#[derive(thiserror::Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum RsProjectError {
    #[error("Failed to build module path from {0}")]
    #[diagnostic(code(GTRSP101))]
    BuildModulePath(String),

    #[error("Failed to resolve definition extensions")]
    #[diagnostic(code(GTRSP201))]
    ExtensionResolve(#[label("This definition")] GtSpan, String),

    #[error("Only structs can be extended with, but {1} is not a struct")]
    #[diagnostic(code(GTRSP202))]
    NonStructExtension(#[label("This struct extensions")] GtSpan, String),

    #[error("Detected cyclic dependencies")]
    #[diagnostic(code(GTRSP203))]
    CyclicExtensions(#[label(collection, "These structs reference each other")] Vec<GtSpan>),

    #[error("Newtypes structs can't be extended with")]
    #[diagnostic(code(GTRSP204))]
    TupleStructExtension(#[label("This struct extensions")] GtSpan),

    #[error("Unit structs can't be extended with")]
    #[diagnostic(code(GTRSP205))]
    UnitStructExtension(#[label("This struct extensions")] GtSpan),
}

impl GtlError for RsProjectError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}
