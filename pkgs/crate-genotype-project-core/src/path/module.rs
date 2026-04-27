//! Module-related paths. It is the fourth level of paths, relative to [GtpSrcDirPath].
//!
//! Module path is the path to Genotype source file. It represents the module id in a project.
//!
//! Types:
//!
//! - [GtpModulePath]: Module path relative to [GtpCwdPath].
//! - [GtpSrcDirRelativeModulePath]: Module path relative to [GtpSrcDirPath].

use crate::prelude::internal::*;

// region: Cwd-relative module path

gtp_cwd_relative_path_wrapper_newtype!(
    /// Module path relative to cwd.
    pub struct GtpModulePath(GtpCwdRelativePath);
);

impl GtpModulePath {
    /// Transforms the cwd-relative module path into a module id that is represented by src-relative
    /// path without the file extension.
    pub fn to_module_id(&self, src_path: &GtpSrcDirPath) -> Result<GtModuleId> {
        let src_relative_path = self
            .relative_path()
            .strip_prefix(&src_path.relative_path())
            .map_err(|_| {
                miette!(
                    "module path '{}' is not inside the src dir '{}'",
                    &self,
                    &src_path
                )
            })?
            .with_extension("");
        let module_id_str = src_relative_path.as_str();
        Ok(GtModuleId(module_id_str.into()))
    }

    #[cfg(feature = "parser")]
    /// Resolves node path to a module path.
    pub fn resolve_path_node(&self, path: &GtPath) -> GtpModulePath {
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
}

// endregion

// region: Src dir-relative module path

gtp_relative_path_wrapper_newtype!(
    /// Module path relative to the src directory.
    pub struct GtpSrcDirRelativeModulePath(GtpSrcDirRelativePath);
    parent: GtpSrcDirPath;
);

impl GtpSrcDirRelativeModulePath {
    #[cfg(feature = "parser")]
    pub fn resolve_path_node(&self, path: &GtPath) -> GtpSrcDirRelativeModulePath {
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

    /// Transforms the src relative path into a package source relative path. It helps targets
    /// generating the correct path for the package source.
    pub fn to_pkg_src_relative_file_path(&self, ext: &'static str) -> GtpPkgSrcDirRelativePath {
        GtpPkgSrcDirRelativePath::new(self.relative_path().with_extension(ext))
    }
}

#[cfg(feature = "parser")]
impl From<&GtpSrcDirRelativeModulePath> for GtModuleId {
    fn from(path: &GtpSrcDirRelativeModulePath) -> Self {
        path.relative_path().with_extension("").as_str().into()
    }
}

#[cfg(feature = "parser")]
impl From<GtpSrcDirRelativeModulePath> for GtModuleId {
    fn from(path: GtpSrcDirRelativeModulePath) -> Self {
        (&path).into()
    }
}

// endregion

#[cfg(test)]
mod tests {
    use genotype_parser::test::Gt;

    use super::*;

    // region: GtpModulePath::to_module_id

    #[test]
    fn test_module_path_to_module_id() {
        let module_path = GtpModulePath::from_str("src/foo/bar.type");
        let module_id = module_path
            .to_module_id(&GtpSrcDirPath::from_str("src"))
            .unwrap();
        assert_equal!(module_id, GtModuleId("foo/bar".into()));
    }

    #[test]
    fn test_module_path_to_module_id_empty_str() {
        let module_path = GtpModulePath::from_str("");
        let module_id = module_path
            .to_module_id(&GtpSrcDirPath::from_str(""))
            .unwrap();
        assert_equal!(module_id, GtModuleId("".into()));
    }

    #[test]
    fn test_module_path_to_module_id_wrong_parent() {
        let module_path = GtpModulePath::from_str("src/foo/bar.type");
        let err = module_path
            .to_module_id(&GtpSrcDirPath::from_str("dist"))
            .unwrap_err();
        assert_debug_snapshot!(err, @r#"
        MietteDiagnostic {
            message: "module path 'src/foo/bar.type' is not inside the src dir 'dist'",
            code: None,
            severity: None,
            help: None,
            url: None,
            labels: None,
        }
        "#);
    }

    // endregion

    // region: GtpModulePath::resolve_path_node

    #[test]
    fn test_module_path_resolve_path_node_simple() {
        let module_path = GtpModulePath::from_str("src/foo/bar.type");
        let path = Gt::path("baz", (0, 0));
        let resolved_path = module_path.resolve_path_node(&path);
        assert_equal!(resolved_path, GtpModulePath::from_str("src/foo/baz.type"));
    }

    #[test]
    fn test_module_path_resolve_path_node_nested() {
        let module_path = GtpModulePath::from_str("src/foo/bar.type");
        let path = Gt::path("baz/qux", (0, 0));
        let resolved_path = module_path.resolve_path_node(&path);
        assert_equal!(
            resolved_path,
            GtpModulePath::from_str("src/foo/baz/qux.type")
        );
    }

    #[test]
    fn test_module_path_resolve_path_node_parent() {
        let module_path = GtpModulePath::from_str("src/foo/bar.type");
        let path = Gt::path("../baz", (0, 0));
        let resolved_path = module_path.resolve_path_node(&path);
        assert_equal!(resolved_path, GtpModulePath::from_str("src/baz.type"));
    }

    // endregion
}
