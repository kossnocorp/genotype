use crate::prelude::internal::*;
use std::process::Command;

pub struct GtbFormatterRunnerSystemKind;

pub trait GtbFormatterRunnerSystem<DiagnosticKind> {}

impl<DiagnosticKind, Type> GtbFormatterRunner<GtbFormatterRunnerSystemKind, DiagnosticKind> for Type
where
    Type: GtbFormatterRunnerSystem<DiagnosticKind> + GtbDiagnosticSink<DiagnosticKind>,
{
    async fn run_formatter(
        &self,
        formatter: &GtpFormatter,
        dist_path: &GtpCwdRelativePath,
    ) -> Result<()> {
        let dist_path_buf = dist_path.to_path_buf();
        let formatter_cmd = formatter.cmd();

        let mut process = Command::new(&formatter_cmd.cmd);
        process.args(&formatter_cmd.args);

        let output = match process.current_dir(&dist_path_buf).output() {
            Ok(output) => output,

            Err(err) => {
                return self
                    .report_formatter_error(&formatter_cmd, dist_path, err.to_string())
                    .await;
            }
        };

        let status = output.status;
        if !status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let details = if stderr.is_empty() { stdout } else { stderr };

            return self
                .report_formatter_error(&formatter_cmd, dist_path, details)
                .await;
        }

        Ok(())
    }
}
