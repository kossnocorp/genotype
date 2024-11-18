use genotype_project::project::GTProject;
use miette::Result;

use crate::source::GTLangProjectSource;

pub trait GTLangProject<Config> {
    fn generate(project: &GTProject, config: Config) -> Result<Self>
    where
        Self: Sized;

    fn render(&self) -> Result<GTLangProjectRender>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectRender {
    pub files: Vec<GTLangProjectSource>,
}
