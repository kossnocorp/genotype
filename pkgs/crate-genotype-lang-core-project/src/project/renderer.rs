use crate::prelude::internal::*;

impl<'project, 'config, ProjectModule: GtlProjectModule>
    GtlProject<'project, 'config, ProjectModule>
{
    pub fn render(&mut self) {
        self.modules = mem::take(&mut self.modules)
            .into_iter()
            .map(|(module_path, module_state)| {
                let new_state = match module_state {
                    state @ GtlProjectModuleState::ConvertError(_)
                    | state @ GtlProjectModuleState::ResolveError(_)
                    | state @ GtlProjectModuleState::RenderError(_) => {
                        // Return self as it is in the error state.
                        state
                    }

                    GtlProjectModuleState::Converted(inner) => inner.to_render_error_state(),

                    GtlProjectModuleState::Rendered(inner) => inner.to_render_error_state(),

                    GtlProjectModuleState::Resolved(inner) => {
                        let source_code = inner.resolved_module.render(self.config.lang_config);
                        match source_code {
                            Ok(source_code) => {
                                GtlProjectModuleState::Rendered(inner.to_rendered(source_code))
                            }

                            Err(err) => {
                                let source_state = inner.to_render_error_source_state();
                                source_state.to_error_state(err)
                            }
                        }
                    }
                };
                (module_path, new_state)
            })
            .collect();
    }
}
