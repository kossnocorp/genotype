use crate::prelude::internal::*;

mod extra;
pub use extra::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtlProjectFile<ProjectModule: GtlProjectModule> {
    Module(GtlProjectModuleState<ProjectModule>),
    Extra(GtlProjectFileExtra),
}

impl<ProjectModule: GtlProjectModule> Into<GtlProjectFile<ProjectModule>>
    for GtlProjectModuleState<ProjectModule>
{
    fn into(self) -> GtlProjectFile<ProjectModule> {
        GtlProjectFile::Module(self)
    }
}
