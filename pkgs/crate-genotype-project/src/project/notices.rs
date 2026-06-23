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

    pub fn as_notices(&self) -> Vec<GtNotice> {
        let mut notices = vec![];
        for module in self.modules.values() {
            match &module {
                GtpModule::Initialized(_source) => {
                    todo!()
                }

                GtpModule::Error(source, err) => {
                    let details = self.source_code_details_for(source);
                    let notice = err.as_notice(&self.config, details);
                    notices.push(notice);
                }

                _ => {}
            }
        }
        notices
    }
}
