use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub enum GtlProjectError {
    #[error("Failed to resolve project")]
    Resolve { error: Box<dyn GtlError> },

    #[error("Failed to generate target file `{path}`")]
    GenerateTargetFile {
        path: GtpTargetFilePath,
        #[source]
        #[diagnostic_source]
        error: Box<dyn GtlError>,
    },
}

impl GtlError for GtlProjectError {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}

impl GtlProjectError {
    pub fn resolve<Error: GtlError>(error: Error) -> Self {
        GtlProjectError::Resolve {
            error: Box::new(error),
        }
    }
}
