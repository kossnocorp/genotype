use crate::prelude::internal::*;

impl GtProject {
    /// Adds module source to the project. It provides map of all module references.
    pub fn add_module_source(&mut self, source: GtpModuleSource) {
        self.module_sources
            .entry(source.path().clone())
            .or_default()
            .insert(source);
    }
}
