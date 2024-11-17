use genotype_project::{module::GTProjectModule, GTProject};
use miette::Result;

pub trait GTLangProjectModule<Config> {
    fn generate(project: &GTProject, module: &GTProjectModule, config: &Config) -> Result<Self>
    where
        Self: Sized;
}
