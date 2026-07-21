use crate::prelude::internal::*;

pub struct GtbFormatterRunnerRemoteKind;

pub trait GtbFormatterRunnerRemote<DiagnosticKind>: GtbRemoteEnv {}

impl<DiagnosticKind, Type> GtbFormatterRunner<GtbFormatterRunnerRemoteKind, DiagnosticKind> for Type
where
    Type: GtbFormatterRunnerRemote<DiagnosticKind> + GtbDiagnosticSink<DiagnosticKind>,
{
    async fn run_formatter(
        &self,
        formatter: &GtpFormatter,
        _dist_path: &GtpCwdRelativePath,
    ) -> Result<()> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::RunFormatter(
                GtbRemoteBackendRequestRunFormatter {
                    formatter: formatter.clone(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::RunFormatter(_) => Ok(()),

            response => Err(miette!(
                "remote run-formatter request returned unexpected response: {response:?}"
            )),
        }
    }
}
