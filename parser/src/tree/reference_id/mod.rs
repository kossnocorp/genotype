use crate::GTSpan;

use super::GTModuleId;

/// Unique project reference id. It allows to identify specific reference in
/// a module or project.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct GTReferenceId(
    /// Module id that contains the reference.
    pub GTModuleId,
    /// Reference position in the source code.
    // [TODO] Consider moving the span to the 0 position for consistency.
    pub  GTSpan,
);
