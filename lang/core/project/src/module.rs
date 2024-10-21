use std::path::PathBuf;

use genotype_project::{module::GTProjectModule, GTProject};

pub trait GTLangProjectModule<Config> {
    fn generate(
        project: &GTProject,
        module: &GTProjectModule,
        config: &Config,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectModuleRender {
    pub path: PathBuf,
    pub code: String,
}
