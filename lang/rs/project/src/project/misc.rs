use crate::prelude::internal::*;

impl RSProject {
    pub fn gitignore_source(&self) -> GTLangProjectSource {
        GTLangProjectSource {
            path: self.config.package_path(".gitignore".into()),
            code: r#"target"#.into(),
        }
    }
}
