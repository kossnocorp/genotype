use crate::prelude::internal::*;

pub struct GtbFsSourceRemoteKind;

pub trait GtbFsSourceRemote: GtbRemoteEnv + GtbFsEnv {}

impl<Type: GtbFsSourceRemote> GtbFsSource<GtbFsSourceRemoteKind> for Type {
    async fn glob_files(&self, path: &GtpCwdRelativePath) -> Result<Vec<GtpCwdRelativePath>> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::GlobFiles(
                GtbRemoteBackendRequestGlobFiles {
                    path: path.to_string(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::GlobFiles(response) => Ok(response
                .paths
                .iter()
                .map(|path| GtpCwdRelativePath::from(path.as_str()))
                .collect()),

            response => Err(miette!(
                "remote glob-files request returned unexpected response: {response:?}"
            )),
        }
    }

    async fn read_file(&self, path: &GtpCwdRelativePath) -> Result<String> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::ReadFile(
                GtbRemoteBackendRequestReadFile {
                    path: path.to_string(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::ReadFile(response) => Ok(response.content),

            response => Err(miette!(
                "remote read-file request returned unexpected response: {response:?}"
            )),
        }
    }

    async fn file_exists(&self, path: &GtpCwdRelativePath) -> Result<bool> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::FileExists(
                GtbRemoteBackendRequestFileExists {
                    path: path.to_string(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::FileExists(response) => Ok(response.exists),

            response => Err(miette!(
                "remote file-exists request returned unexpected response: {response:?}"
            )),
        }
    }

    async fn is_file(&self, path: &GtpCwdRelativePath) -> Result<bool> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::IsFile(
                GtbRemoteBackendRequestIsFile {
                    path: path.to_string(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::IsFile(response) => Ok(response.is_file),

            response => Err(miette!(
                "remote is-file request returned unexpected response: {response:?}"
            )),
        }
    }

    async fn find_file(&self, file_name: &str) -> Result<GtpCwdRelativePath> {
        match self
            .remote_interop()
            .send_request(GtbRemoteBackendRequest::FindFile(
                GtbRemoteBackendRequestFindFile {
                    file_name: file_name.to_string(),
                },
            ))
            .await?
        {
            GtbRemoteBackendRequestResponse::FindFile(response) => {
                Ok(GtpCwdRelativePath::from(response.path.as_str()))
            }

            response => Err(miette!(
                "remote find-file request returned unexpected response: {response:?}"
            )),
        }
    }
}
