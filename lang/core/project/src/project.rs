use std::path::PathBuf;

use genotype_project::project::GTProject;

use crate::module::GTLangProjectModuleRender;

pub trait GTLangProject<Options> {
    fn generate(project: &GTProject, out: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn render(&self, options: &Options) -> Result<GTLangProjectRender, Box<dyn std::error::Error>>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectRender {
    pub root: PathBuf,
    pub modules: Vec<GTLangProjectModuleRender>,
}
