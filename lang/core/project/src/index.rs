use crate::prelude::internal::*;

pub trait GtlProject<'a> {
    type Module: GtlProjectModule<Self::LangConfig>;

    type LangConfig: GtlConfig;

    fn generate(project: &'a GtProject) -> Result<Self>
    where
        Self: Sized;

    fn dist(&self) -> Result<GtlProjectDist>;

    fn modules(&self) -> Vec<Self::Module>;

    fn dependencies(
        &'a self,
    ) -> Vec<<<Self as GtlProject<'a>>::Module as GtlProjectModule<Self::LangConfig>>::Dependency>
    {
        self.modules()
            .iter()
            .flat_map(|module| module.dependencies())
            .collect::<IndexSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GtlProjectDist {
    pub files: Vec<GtlProjectFile>,
}
