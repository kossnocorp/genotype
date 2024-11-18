use genotype_lang_core_project::source::GTLangProjectSource;

use super::RSProject;

impl RSProject {
    pub fn gitignore_source(&self) -> GTLangProjectSource {
        GTLangProjectSource {
            path: self.config.package_path(".gitignore".into()),
            code: r#"target"#.into(),
        }
    }
}
