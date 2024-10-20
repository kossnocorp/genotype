use std::path::PathBuf;

use genotype_config::GTConfig;
use genotype_project::{module::GTProjectModule, GTProject};

pub trait GTLangProjectModule {
    fn generate(
        project: &GTProject,
        module: &GTProjectModule,
        config: &GTConfig,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Clone)]
pub struct GTLangProjectModuleRender {
    pub path: PathBuf,
    pub code: String,
}
