use crate::prelude::internal::*;

/// Module path relative to the src directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GtModulePath(GtSrcRelativePath);

impl GtModulePath {
    #[cfg(feature = "parser")]
    pub fn resolve(&self, path: &GTPath) -> GtModulePath {
        let parent_path = if let Some(parent) = self.0.relative_path().parent() {
            parent
        } else {
            &RelativePathBuf::from("")
        };
        Self::new(
            parent_path
                .join_normalized(path.source_str())
                .with_extension("type"),
        )
    }

    pub fn as_src_relative_path(self) -> GtSrcRelativePath {
        self.0
    }

    /// Transforms the src relative path into a package source relative path. It helps targets
    /// generating the correct path for the package source.
    pub fn to_pkg_src_relative_path(&self, ext: &'static str) -> GtPkgSrcRelativePath {
        GtPkgSrcRelativePath::new(self.relative_path().with_extension(ext))
    }
}

impl GtRelativePath for GtModulePath {
    fn new(path: RelativePathBuf) -> Self {
        Self(GtSrcRelativePath::new(path))
    }

    fn relative_path(&self) -> &RelativePathBuf {
        self.0.relative_path()
    }
}

impl From<GtModulePath> for GtSrcRelativePath {
    fn from(path: GtModulePath) -> Self {
        path.0
    }
}

impl From<&str> for GtModulePath {
    fn from(path: &str) -> Self {
        Self::new(path.into())
    }
}

#[cfg(feature = "parser")]
impl From<&GtModulePath> for GTModuleId {
    fn from(path: &GtModulePath) -> Self {
        path.relative_path().with_extension("").as_str().into()
    }
}

#[cfg(feature = "parser")]
impl From<GtModulePath> for GTModuleId {
    fn from(path: GtModulePath) -> Self {
        (&path).into()
    }
}
