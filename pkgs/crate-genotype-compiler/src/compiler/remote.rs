use crate::prelude::internal::*;

/// Remote compiler.
pub struct GtcRemote {
    state: GtCompilerState,
    backend: GtbRemote,
}

impl GtcRemote {
    pub async fn handle_runtime_request(
        &mut self,
        request: GtcRemoteRuntimeRequest,
    ) -> Result<GtcRemoteRuntimeRequestResponse> {
        match request {
            GtcRemoteRuntimeRequest::LoadInProject(GtcRemoteRuntimeRequestLoadInProject {
                ..
            }) => {
                let meta = self.load_in_project().await?;
                Ok(GtcRemoteRuntimeRequestResponse::LoadInProject(
                    GtcRemoteRuntimeRequestResponseLoadInProject { meta },
                ))
            }

            GtcRemoteRuntimeRequest::LoadInModules(GtcRemoteRuntimeRequestLoadInModules {
                ..
            }) => {
                let meta = self.load_in_modules().await?;
                Ok(GtcRemoteRuntimeRequestResponse::LoadInModules(
                    GtcRemoteRuntimeRequestResponseLoadInModules { meta },
                ))
            }

            GtcRemoteRuntimeRequest::Compile(GtcRemoteRuntimeRequestCompile { .. }) => {
                let meta = self.compile().await?;
                Ok(GtcRemoteRuntimeRequestResponse::Compile(
                    GtcRemoteRuntimeRequestResponseCompile { meta },
                ))
            }
        }
    }
}

impl GtCompiler<GtcRemoteProps> for GtcRemote {
    fn new(props: GtcRemoteProps) -> Result<Self> {
        let (interop, cwd_path, base_path, config_path) = props;

        let backend = GtbRemote::new(interop, cwd_path, base_path);

        Ok(Self {
            state: GtCompilerState::New {
                config_path,
                meta: GtcMetaNew {},
            },
            backend,
        })
    }

    fn state(&self) -> &GtCompilerState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut GtCompilerState {
        &mut self.state
    }

    fn backend(&self) -> &impl GtBackend {
        &self.backend
    }
}

pub type GtcRemoteProps = (
    Box<dyn GtbRemoteInterop>,
    GtpCwdPath,
    GtpCwdRelativePath,
    Option<GtpCwdRelativePath>,
);
