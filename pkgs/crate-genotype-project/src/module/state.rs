use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtProjectModuleState {
    /// Module failed to load.
    Error(GtProjectModuleError),
    Parsed(GtProjectModule),
}
