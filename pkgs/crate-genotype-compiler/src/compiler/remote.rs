use crate::prelude::internal::*;

/// Remote compiler.
pub struct GtcRemote {
    state: GtCompilerState,
    meta: GtcMetaState,
    backend: GtbRemote,
}

impl GtcRemote {
    pub async fn handle_runtime_request(
        &mut self,
        request: GtbRemoteRuntimeRequest,
    ) -> Result<GtbRemoteRuntimeRequestResponse> {
        match request {
            GtbRemoteRuntimeRequest::LoadInProject(GtbRemoteRuntimeRequestLoadInProject {
                ..
            }) => {
                self.load_in_project().await?;
                Ok(GtbRemoteRuntimeRequestResponse::LoadInProject(
                    GtbRemoteRuntimeRequestResponseLoadInProject {},
                ))
            }

            GtbRemoteRuntimeRequest::LoadInModules(GtbRemoteRuntimeRequestLoadInModules {
                ..
            }) => {
                self.load_in_modules().await?;
                Ok(GtbRemoteRuntimeRequestResponse::LoadInModules(
                    GtbRemoteRuntimeRequestResponseLoadInModules {},
                ))
            }

            GtbRemoteRuntimeRequest::Compile(GtbRemoteRuntimeRequestCompile { .. }) => {
                let meta = self.compile().await?;
                Ok(GtbRemoteRuntimeRequestResponse::Compile(
                    GtbRemoteRuntimeRequestResponseCompile { meta },
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
            state: GtCompilerState::New { config_path },
            meta: GtcMetaState::new(),
            backend,
        })
    }

    fn state(&self) -> &GtCompilerState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut GtCompilerState {
        &mut self.state
    }

    fn meta(&self) -> &GtcMetaState {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut GtcMetaState {
        &mut self.meta
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
