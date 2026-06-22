use crate::prelude::internal::*;

mod parse;
pub use parse::*;

mod source;
pub use source::*;

mod resolve;
pub use resolve::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GtModule {
    /// Module identifier. Used to reference the module in the project.
    pub id: GtModuleId,
    #[visit]
    pub doc: Option<GtDoc>,
    #[visit]
    pub imports: Vec<GtImport>,
    #[visit]
    pub aliases: Vec<GtAlias>,
}
