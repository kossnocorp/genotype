use genotype_config::GTConfig;
use genotype_project::project::GTProject;

use crate::module::GTLangProjectModuleRender;

pub trait GTLangProject {
    fn generate(project: &GTProject, config: &GTConfig) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn render(&self, config: &GTConfig) -> Result<GTLangProjectRender, Box<dyn std::error::Error>>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectRender {
    pub modules: Vec<GTLangProjectModuleRender>,
}
