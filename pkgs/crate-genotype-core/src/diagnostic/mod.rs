use crate::prelude::internal::*;

mod content;

impl GtDiagnostic {
    pub fn title(&self) -> &str {
        match &self.content {
            GtDiagnosticContent::Message(GtDiagnosticContentMessage { title, .. }) => title,

            GtDiagnosticContent::Report(GtDiagnosticContentReport { title, .. }) => title,
        }
    }

    pub fn error<Content: Into<GtDiagnosticContent>>(content: Content) -> Self {
        GtDiagnostic {
            kind: GtDiagnosticKind::Error,
            content: content.into(),
        }
    }

    pub fn warning<Content: Into<GtDiagnosticContent>>(content: Content) -> Self {
        GtDiagnostic {
            kind: GtDiagnosticKind::Warning,
            content: content.into(),
        }
    }

    pub fn success<Content: Into<GtDiagnosticContent>>(content: Content) -> Self {
        GtDiagnostic {
            kind: GtDiagnosticKind::Success,
            content: content.into(),
        }
    }

    pub fn info<Content: Into<GtDiagnosticContent>>(content: Content) -> Self {
        GtDiagnostic {
            kind: GtDiagnosticKind::Info,
            content: content.into(),
        }
    }

    pub fn format_report(report: Report) -> String {
        format!("{report:?}")
    }
}
