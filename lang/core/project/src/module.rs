use genotype_lang_core_tree::*;
use genotype_project::*;
use miette::Result;

pub trait GtlProjectModule {
    type Dependency: GtlDependencyIdent;

    fn generate(project: &GtProject, module: &GTProjectModule) -> Result<Self>
    where
        Self: Sized;

    fn dependencies(&self) -> Vec<Self::Dependency>;
}
