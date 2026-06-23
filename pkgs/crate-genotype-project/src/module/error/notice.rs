use crate::prelude::internal::*;

impl GtpModuleError {
    pub fn as_notice(&self, config: &GtpConfig, details: Vec<GtpModuleErrorDetails>) -> GtNotice {
        let err_str = format!("{self}");
        match self {
            GtpModuleError::Init { .. } => {
                let reports = Self::format_import_reports(config, details);
                Self::notice_with_reports(err_str, reports)
            }

            GtpModuleError::Read { .. } => {
                let reports = Self::format_import_reports(config, details);
                Self::notice_with_reports(err_str, reports)
            }

            GtpModuleError::Parse {
                path,
                error,
                source_code,
            } => {
                let named_source = NamedSource::new(path.to_string(), source_code.clone());
                error.as_notice(path.as_str(), named_source)
            }

            GtpModuleError::Resolve { error, .. } => {
                let mut reports = vec![GtNotice::format_report(Report::new(error.clone()))];
                reports.extend(Self::format_import_reports(config, details));
                Self::notice_with_reports(err_str, reports)
            }

            GtpModuleError::ResolveInitialized => {
                let reports = Self::format_import_reports(config, details);
                Self::notice_with_reports(err_str, reports)
            }
        }
    }

    fn notice_with_reports(title: String, reports: Vec<String>) -> GtNotice {
        if reports.is_empty() {
            GtNotice::error(title)
        } else {
            GtNotice::error((title, reports))
        }
    }

    fn format_import_reports(
        config: &GtpConfig,
        details: Vec<GtpModuleErrorDetails>,
    ) -> Vec<String> {
        Self::format_reports(
            config,
            details,
            "Module imported by another module",
            "Imported here",
        )
    }

    fn format_reports(
        config: &GtpConfig,
        details: Vec<GtpModuleErrorDetails>,
        message: &str,
        label: &str,
    ) -> Vec<String> {
        let mut reports = vec![];
        for (source, span_with_source) in details.iter() {
            match source {
                GtpModuleSource::Entry { .. } => {
                    let report = miette!(
                        "Module matched entry pattern '{entry}'",
                        entry = config.entry
                    );
                    reports.push(format!("{report:?}"));
                }

                GtpModuleSource::Dependency { .. } => {
                    if let Some((span, Some(source_code))) = span_with_source {
                        let report =
                            miette!(labels = vec![LabeledSpan::at(**span, label)], "{message}")
                                .with_source_code(source_code.clone());
                        reports.push(format!("{report:?}"));
                    }
                }
            }
        }
        reports
    }
}

pub type GtpModuleErrorDetails<'a> = (
    &'a GtpModuleSource,
    Option<(&'a GtSpan, Option<NamedSource<String>>)>,
);
