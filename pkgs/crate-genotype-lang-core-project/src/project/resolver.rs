use crate::prelude::internal::*;

impl<'project, 'config, ProjectModule: GtlProjectModule>
    GtlProject<'project, 'config, ProjectModule>
{
    pub fn resolve(&mut self) -> Result<(), GtlProjectError> {
        ProjectModule::resolve_modules(self.config.lang_config, &mut self.modules);
        Ok(())
    }
}
