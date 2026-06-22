use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq, Error, Diagnostic, Serialize)]
#[error("Module source state is {}", self.is_message_chunk())]
pub enum GtlProjectModuleConvertErrorSourceState {
    Initialized,
    Error(#[source] GtpModuleError),
    Parsed,
}

impl GtlProjectModuleConvertErrorSourceState {
    fn is_message_chunk(&self) -> &str {
        match self {
            Self::Initialized => "initialized but not parsed",
            Self::Error(_) => "in error state",
            Self::Parsed => "parsed but not generated",
        }
    }
}
