use crate::prelude::internal::*;

/// Remote runtime. It combines serial project loader, message-based file access and stdio
/// diagnostics reporting.
pub struct GtbRemote {
    /// Remote interop implementation for the environment.
    interop: Box<dyn GtbRemoteInterop>,

    /// Current working directory path.
    cwd_path: GtpCwdPath,

    /// Base path for the project source to resolve relative file paths.
    base_path: GtpCwdRelativePath,
}

impl GtbRemote {
    /// Creates a new remote backend.
    pub fn new(
        interop: Box<dyn GtbRemoteInterop>,
        cwd_path: GtpCwdPath,
        base_path: GtpCwdRelativePath,
    ) -> Self {
        Self {
            interop,
            cwd_path,
            base_path,
        }
    }
}

impl GtbFsEnv for GtbRemote {
    /// Returns the cwd path.
    fn cwd_path(&self) -> &GtpCwdPath {
        &self.cwd_path
    }

    /// Returns the base project directory to resolve relative file paths.
    fn base_path(&self) -> &GtpCwdRelativePath {
        &self.base_path
    }
}

impl GtbFsSourceRemote for GtbRemote {}

impl GtbFsSinkRemote for GtbRemote {}

impl GtbDiagnosticSinkRemote for GtbRemote {}

impl GtbRemoteEnv for GtbRemote {
    fn remote_interop(&self) -> &dyn GtbRemoteInterop {
        &*self.interop
    }
}

impl GtbFormatterRunnerRemote<GtbDiagnosticSinkRemoteKind> for GtbRemote {}

impl GtBackend for GtbRemote {
    type FileSourceKind = GtbFsSourceRemoteKind;

    type FileSinkKind = GtbFsSinkRemoteKind;

    type DiagnosticSinkKind = GtbDiagnosticSinkRemoteKind;

    type FormatterRunnerKind = GtbFormatterRunnerRemoteKind;
}
