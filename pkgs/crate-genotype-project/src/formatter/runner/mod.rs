use crate::prelude::internal::*;

mod system;
pub use system::*;

pub trait GtpFormatterRunner<Kind, DiagnosticSink>: GtpDiagnosticSink<DiagnosticSink> {
    fn run_formatter(&self, formatter: &GtpFormatter, path: &GtpCwdRelativePath) -> Result<()>;

    fn report_formatter_error(
        &self,
        cmd: &GtpFormatterCmd,
        dist_path: &GtpCwdRelativePath,
        details: String,
    ) -> Result<()> {
        self.report_diagnostic(&GtDiagnostic::warning((
            format!("Failed to run `{cmd}` formatter in `{dist_path}`"),
            details,
        )));
        Ok(())
    }
}
