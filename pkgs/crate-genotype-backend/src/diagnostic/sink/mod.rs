use crate::prelude::internal::*;

mod stdio;
pub use stdio::*;

mod remote;
pub use remote::*;

#[allow(async_fn_in_trait)]
pub trait GtbDiagnosticSink<Kind> {
    async fn report_diagnostics(&self, diagnostics: &[GtDiagnostic]) -> Result<()> {
        for diagnostic in diagnostics {
            self.report_diagnostic(diagnostic).await?;
        }
        Ok(())
    }

    async fn report_diagnostic(&self, diagnostic: &GtDiagnostic) -> Result<()>;
}
