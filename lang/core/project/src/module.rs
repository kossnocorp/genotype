use crate::prelude::internal::*;

pub trait GtlProjectModule<LangConfig: GtlConfig> {
    type Dependency: GtlDependencyIdent;

    fn generate(config: &LangConfig, module: &GtProjectModule) -> Result<Self>
    where
        Self: Sized;

    fn dependencies(&self) -> Vec<Self::Dependency>;
}
