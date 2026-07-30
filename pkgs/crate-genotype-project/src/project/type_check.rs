use crate::prelude::internal::*;

impl GtProject {
    /// Type checks the resolved project modules.
    pub fn type_check_modules(&mut self) -> Result<()> {
        let resolved_modules = self
            .modules
            .iter()
            .filter_map(|(path, module)| match module {
                GtpModule::Resolved(module) => Some((path.clone(), (**module).clone())),
                _ => None,
            })
            .collect();

        for module in self.modules.values_mut() {
            let current_module =
                std::mem::replace(module, GtpModule::Initialized(module.source().clone()));
            *module = current_module.type_check(&resolved_modules);
        }

        Ok(())
    }
}
