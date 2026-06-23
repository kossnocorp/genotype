use crate::prelude::internal::*;

mod extra;
pub use extra::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum GtlProjectFile<ProjectModule: GtlProjectModule> {
    Module(GtlProjectModuleState<ProjectModule>),
    Extra(GtlProjectFileExtra),
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectModuleState<ProjectModule>>
    for GtlProjectFile<ProjectModule>
{
    fn from(val: GtlProjectModuleState<ProjectModule>) -> Self {
        GtlProjectFile::Module(val)
    }
}
