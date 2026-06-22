use crate::prelude::internal::*;

impl<'project, 'config, ProjectModule: GtlProjectModule>
    GtlProject<'project, 'config, ProjectModule>
{
    pub fn convert(&mut self, modules: &IndexMap<GtpModulePath, GtpModule>) {
        self.modules = modules
            .iter()
            .map(|(module_path, project_module)| {
                let source_path = project_module.source().path().clone();

                let target_path_result = self.config.module_target_file_path(&source_path);
                let lang_project_module = match target_path_result {
                    Ok(target_path) => match project_module {
                        GtpModule::Initialized(source) => {
                            GtlProjectModuleConvertError::SourceState {
                                source: source.clone(),
                                source_state: GtlProjectModuleConvertErrorSourceState::Initialized,
                                target_path,
                            }
                            .into()
                        }

                        GtpModule::Error(module_source, error) => {
                            GtlProjectModuleConvertError::SourceState {
                                source: module_source.clone(),
                                source_state: GtlProjectModuleConvertErrorSourceState::Error(
                                    error.clone(),
                                ),
                                target_path,
                            }
                            .into()
                        }

                        GtpModule::Parsed(module) => GtlProjectModuleConvertError::SourceState {
                            source: module.source.clone(),
                            source_state: GtlProjectModuleConvertErrorSourceState::Parsed,
                            target_path,
                        }
                        .into(),

                        GtpModule::Resolved(module_resolved) => {
                            match ProjectModule::convert(self.config.lang_config, module_resolved) {
                                Ok(module) => GtlProjectModuleConverted {
                                    source_path,
                                    target_path,
                                    project_module: module,
                                }
                                .into(),

                                Err(error) => GtlProjectModuleConvertError::ConvertError {
                                    source_path,
                                    target_path,
                                    error: error.clone_box(),
                                }
                                .into(),
                            }
                        }
                    },

                    Err(error) => {
                        GtlProjectModuleConvertError::ResolvePath { source_path, error }.into()
                    }
                };

                (module_path.clone(), lang_project_module)
            })
            .collect::<IndexMap<_, _>>();
    }
}
