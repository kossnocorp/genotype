use crate::prelude::internal::*;

pub trait GtlProjectModule<'a, LangConfig: GtlConfig> {
    type Dependency: GtlDependencyIdent;

    fn generate(config: &'a GtConfigPkg<'a, LangConfig>, module: &GTProjectModule) -> Result<Self>
    where
        Self: Sized;

    fn dependencies(&self) -> Vec<Self::Dependency>;
}
