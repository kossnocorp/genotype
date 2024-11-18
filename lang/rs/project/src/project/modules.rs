use genotype_lang_core_project::source::GTLangProjectSource;
use genotype_lang_rs_tree::{rs_indent, RSRender};
use miette::Result;

use super::RSProject;

impl RSProject {
    pub fn modules_source(&self) -> Result<Vec<GTLangProjectSource>> {
        self.modules
            .iter()
            .map(
                |module| match module.module.render(&rs_indent(), &self.config.lang) {
                    Ok(code) => Ok(GTLangProjectSource {
                        path: module.path.clone(),
                        code,
                    }),
                    Err(err) => Err(err),
                },
            )
            .collect::<Result<Vec<_>>>()
    }
}
