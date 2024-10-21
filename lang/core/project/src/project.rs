use genotype_project::project::GTProject;

use crate::module::GTLangProjectModuleRender;

pub trait GTLangProject<Config> {
    fn generate(project: &GTProject, config: &Config) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn render(&self, config: &Config) -> Result<GTLangProjectRender, Box<dyn std::error::Error>>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectRender {
    pub modules: Vec<GTLangProjectModuleRender>,
}
