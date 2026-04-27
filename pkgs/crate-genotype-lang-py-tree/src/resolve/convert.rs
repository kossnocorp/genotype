use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct PyConvertResolve {
    pub paths: IndexMap<GtPath, GtPath>,
    pub globs: IndexMap<GtPath, String>,
    pub identifiers: IndexMap<GtIdentifier, GtIdentifier>,
    pub imported: IndexSet<GtIdentifier>,
}
