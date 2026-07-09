use crate::prelude::internal::*;

pub struct GtbFsSinkRemoteKind;

pub trait GtbFsSinkRemote: GtbRemoteEnv {}

impl<Type: GtbFsSinkRemote> GtbFsSink<GtbFsSinkRemoteKind> for Type {
    async fn write_file(&self, path: &GtpCwdRelativePath, content: &str) -> Result<()> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::WriteFile(
                GtbRemoteBackendRequestWriteFile {
                    path: path.to_string(),
                    content: content.to_owned(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::WriteFile(_) => Ok(()),

            response => Err(miette!(
                "remote write-file request returned unexpected response: {response:?}"
            )),
        }
    }
}
