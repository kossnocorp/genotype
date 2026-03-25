use genotype_parser::*;
use serde::Serialize;

/// Project module identifier resolve data.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtpModuleIdentifierResolve {
    /// Where the identifier is defined.
    pub source: GtpModuleIdentifierSource,
}

/// Describes where the module identifier is defined.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtpModuleIdentifierSource {
    /// Locally defined identifier.
    Local,
    /// Externally defined identifier.
    External(
        /// Path to the module that contains the identifier.
        GtPath,
    ),
    /// Package identifier.
    Package(
        /// Path to the package that contains the identifier.
        GtPath,
    ),
}
