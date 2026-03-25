use crate::prelude::internal::*;

/// Unique project reference id. It allows to identify specific reference in
/// a module or project.
#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize)]
pub struct GtReferenceId(
    /// Module id that **contains** the reference.
    pub GtModuleId,
    /// Reference position in the source code.
    // [TODO] Move the span to the 0 position for consistency.
    pub  GtSpan,
);
