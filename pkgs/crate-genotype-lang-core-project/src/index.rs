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
        let mut dependencies = vec![];
        for module in self.modules().iter() {
            for dependency in module.dependencies() {
                if !dependencies.contains(&dependency) {
                    dependencies.push(dependency);
                }
            }
        }
        dependencies
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlProjectDist {
    pub files: Vec<GtlProjectFile>,
}
