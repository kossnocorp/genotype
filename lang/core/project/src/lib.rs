use genotype_project::*;
use indexmap::IndexSet;
use miette::Result;

mod module;
pub use module::*;

mod file;
pub use file::*;

mod manifest;
pub use manifest::*;

mod error;
pub use error::*;

pub mod prelude;

pub trait GtlProject<'a> {
    type Module: GtlProjectModule;

    fn generate(project: &'a GtProject) -> Result<Self>
    where
        Self: Sized;

    fn out(&self) -> Result<GtlProjectOut>;

    fn modules(&self) -> Vec<Self::Module>;

    fn dependencies(
        &'a self,
    ) -> Vec<<<Self as GtlProject<'a>>::Module as GtlProjectModule>::Dependency> {
        self.modules()
            .iter()
            .flat_map(|module| module.dependencies())
            .collect::<IndexSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GtlProjectOut {
    pub files: Vec<GtlProjectFile>,
}
