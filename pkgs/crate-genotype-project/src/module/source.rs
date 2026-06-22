use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub enum GtpModuleSource {
    Entry {
        path: GtpModulePath,
    },

    Dependency {
        path: GtpModulePath,
        parent_path: GtpModulePath,
        parent_span: GtSpan,
    },
}

impl GtpModuleSource {
    pub fn path(&self) -> &GtpModulePath {
        match self {
            Self::Entry { path } | Self::Dependency { path, .. } => path,
        }
    }
}

impl From<&GtpModuleSource> for GtpModuleSource {
    fn from(source: &GtpModuleSource) -> Self {
        source.clone()
    }
}

impl From<GtpModulePath> for GtpModuleSource {
    fn from(path: GtpModulePath) -> Self {
        GtpModuleSource::Entry { path }
    }
}

impl From<&GtpModulePath> for GtpModuleSource {
    fn from(path: &GtpModulePath) -> Self {
        path.clone().into()
    }
}
