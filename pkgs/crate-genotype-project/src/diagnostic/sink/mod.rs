use crate::prelude::internal::*;

mod stdio;
pub use stdio::*;

pub trait GtpDiagnosticSink<Kind> {
    fn report_diagnostics(&self, diagnostics: &[GtDiagnostic]) {
        for diagnostic in diagnostics {
            self.report_diagnostic(diagnostic);
        }
    }

    fn report_diagnostic(&self, diagnostic: &GtDiagnostic);
}
