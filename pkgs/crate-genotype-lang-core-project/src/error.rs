use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub enum GtlProjectError {
    #[error("Failed to resolve project")]
    Resolve { error: Box<dyn GtlError> },
}

impl GtlProjectError {
    pub fn resolve<Error: GtlError>(error: Error) -> Self {
        GtlProjectError::Resolve {
            error: Box::new(error),
        }
    }
}
