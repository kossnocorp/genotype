use std::fmt::Display;

use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
pub struct GtlErrorMessage(pub String);

impl Display for GtlErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GtlError for GtlErrorMessage {
    fn clone_box(&self) -> Box<dyn GtlError> {
        Box::new(self.clone())
    }
}

impl<Str: AsRef<str>> From<Str> for Box<dyn GtlError> {
    fn from(message: Str) -> Self {
        Box::new(GtlErrorMessage(message.as_ref().to_string()))
    }
}
