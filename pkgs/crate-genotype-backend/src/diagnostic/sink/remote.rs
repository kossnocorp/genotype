use crate::prelude::internal::*;

pub struct GtbDiagnosticSinkRemoteKind;

pub trait GtbDiagnosticSinkRemote: GtbRemoteEnv {}

impl<Type: GtbDiagnosticSinkRemote> GtbDiagnosticSink<GtbDiagnosticSinkRemoteKind> for Type {
    async fn report_diagnostic(&self, diagnostic: &GtDiagnostic) -> Result<()> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::ReportDiagnostic(
                GtbRemoteBackendRequestReportDiagnostic {
                    diagnostic: diagnostic.clone(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::ReportDiagnostic(_) => Ok(()),

            response => Err(miette!(
                "remote report-diagnostic request returned unexpected response: {response:?}"
            )),
        }
    }
}
