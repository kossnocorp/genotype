use std::sync::Arc;

use super::GTPModulePath;

/// Project module path resolve data.
#[derive(Debug, PartialEq, Clone)]
pub struct GTPModulePathResolve {
    /// Associated module path.
    module_path: Arc<GTPModulePath>,
}
