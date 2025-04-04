use super::GTPModulePath;

/// Project module path resolve data.
#[derive(Debug, PartialEq, Clone)]
pub struct GTPModulePathResolve {
    /// Associated module path.
    pub module_path: GTPModulePath,
}
