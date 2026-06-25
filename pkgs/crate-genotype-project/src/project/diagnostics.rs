use crate::prelude::internal::*;

impl GtProject {
    fn source_code_details_for<'a>(
        &'a self,
        source: &'a GtpModuleSource,
    ) -> Vec<GtpModuleErrorDetails<'a>> {
        self.module_sources
            .get(source.path())
            .map(|sources| {
                sources
                    .iter()
                    .map(|source| {
                        let details = match source {
                            GtpModuleSource::Entry { .. } => None,

                            GtpModuleSource::Dependency {
                                path: _,
                                parent_path,
                                parent_span,
                            } => self.modules.get(parent_path).map(|module| {
                                let source_code = module.source_code().map(|code| {
                                    NamedSource::new(parent_path.to_string(), code.to_string())
                                });
                                (parent_span, source_code)
                            }),
                        };
                        (source, details)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    pub fn as_final_diagnostics(&self) -> Vec<GtDiagnostic> {
        let mut diagnostics = vec![];
        for module in self.modules.values() {
            match &module {
                GtpModule::Resolved(_) => {
                    // Resolved, the expected final state, no diagnostic needed.
                }

                GtpModule::Error(source, err) => {
                    let details = self.source_code_details_for(source);
                    let diagnostic = err.as_diagnostic(&self.config, details);
                    diagnostics.push(diagnostic);
                }

                GtpModule::Parsed(_) | GtpModule::Initialized(_) => {
                    diagnostics.push(Self::invalid_state_error_diagnostic(module))
                }
            }
        }
        diagnostics
    }

    fn invalid_state_error_diagnostic(module: &GtpModule) -> GtDiagnostic {
        let source = module.source();
        GtDiagnostic::error(format!(
            "Module `{path}` is in an invalid state \"{state}\"",
            path = source.path(),
            state = module.state_name()
        ))
    }
}
