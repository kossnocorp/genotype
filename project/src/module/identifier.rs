use genotype_parser::*;

/// Project module identifier resolve data.
#[derive(Debug, PartialEq, Clone)]
pub struct GTPModuleIdentifierResolve {
    /// Where the identifier is defined.
    pub source: GTPModuleIdentifierSource,
}

/// Describes where the module identifier is defined.
#[derive(Debug, PartialEq, Clone)]
pub enum GTPModuleIdentifierSource {
    /// Locally defined identifier.
    Local,
    /// Externally defined identifier.
    External(
        /// Path to the module that contains the identifier.
        GTPath,
    ),
}
