use crate::prelude::internal::*;

impl<'project, 'config, ProjectModule: GtlProjectModule>
    GtlProject<'project, 'config, ProjectModule>
{
    pub fn render(&mut self) -> Result<(), GtlProjectError> {
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
                        let source_code = inner.resolved_module.render(self.config.lang_config());
                        match source_code {
                            Ok(source_code) => {
                                let source_code = if self.config.warning_comment() {
                                    let warning =
                                        self.render_generated_warning(&inner.converted)?;
                                    format!("{warning}\n\n{source_code}")
                                } else {
                                    source_code
                                };
                                GtlProjectModuleState::Rendered(inner.to_rendered(source_code))
                            }

                            Err(err) => {
                                let source_state = inner.to_render_error_source_state();
                                source_state.to_error_state(err)
                            }
                        }
                    }
                };
                Ok((module_path, new_state))
            })
            .collect::<Result<_, GtlProjectError>>()?;
        Ok(())
    }

    fn render_generated_warning(
        &self,
        converted: &GtlProjectModuleConverted<ProjectModule>,
    ) -> Result<String, GtlProjectError> {
        let target_dir = converted
            .target_path
            .relative_path()
            .parent()
            .ok_or_else(|| GtlProjectError::GenerateTargetFile {
                path: converted.target_path.clone(),
                error: "Failed to resolve the target file parent directory".into(),
            })?;

        let source_path = target_dir.relative(converted.source_path.relative_path());
        let content = format!("Do not edit manually! Code generated from {source_path}",);
        self.config.project_version();
        let comment = self.config.lang_config().comment_line(&content);
        Ok(comment)
    }
}
