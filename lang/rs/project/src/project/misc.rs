use crate::prelude::internal::*;

impl RsProject<'_> {
    pub fn gitignore_source(&self) -> GtlProjectFile {
        GtlProjectFile {
            path: self.project.config.rs.package_path(".gitignore".into()),
            source: r#"target"#.into(),
        }
    }
}
