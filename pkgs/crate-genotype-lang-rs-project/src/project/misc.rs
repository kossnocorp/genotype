use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn gitignore_source(&self) -> GtlProjectFile {
        GtlProjectFile {
            path: self
                .config
                .pkg_path()
                .join_str_as_cwd_relative_path(".gitignore"),
            source: r#"target"#.into(),
        }
    }
}
