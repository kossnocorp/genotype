use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn gitignore_source(&self) -> GtlProjectFile {
        GtlProjectFile {
            path: self
                .config
                .pkg_path()
                .join_as_cwd_relative_path(&".gitignore".into()),
            source: r#"target"#.into(),
        }
    }
}
