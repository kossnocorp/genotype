use crate::prelude::internal::*;

mod stdio;
pub use stdio::*;

pub trait GtcDiagnosticSink {
    fn print_diagnostics(&self, diagnostics: Vec<GtDiagnostic>) {
        for diagnostic in diagnostics {
            self.print_diagnostic(diagnostic);
        }
    }

    fn print_diagnostic(&self, diagnostic: GtDiagnostic);
}
