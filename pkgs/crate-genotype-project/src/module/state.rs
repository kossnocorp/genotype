use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtpModuleState {
    /// Module is currently being loaded.
    Loading,
    /// Module failed to load.
    Error(GtpModuleError),
    /// Module has been parsed successfully.
    Parsed(GtpModule),
}
