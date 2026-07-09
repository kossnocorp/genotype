use crate::prelude::internal::*;

mod system;
pub use system::*;

mod remote;
pub use remote::*;

#[allow(async_fn_in_trait)]
pub trait GtbFormatterRunner<Kind, DiagnosticSink>: GtbDiagnosticSink<DiagnosticSink> {
    async fn run_formatter(
        &self,
        formatter: &GtpFormatter,
        path: &GtpCwdRelativePath,
    ) -> Result<()>;

    async fn report_formatter_error(
        &self,
        cmd: &GtpFormatterCmd,
        dist_path: &GtpCwdRelativePath,
        details: String,
    ) -> Result<()> {
        self.report_diagnostic(&GtDiagnostic::warning((
            format!("Failed to run `{cmd}` formatter in `{dist_path}`"),
            details,
        )))
        .await
    }
}
