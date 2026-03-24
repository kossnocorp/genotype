use crate::prelude::internal::*;

mod parse;
pub use parse::*;

mod resolve;
pub use resolve::*;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct GTModule {
    /// Module identifier. Used to reference the module in the project.
    pub id: GTModuleId,
    #[visit]
    pub doc: Option<GTDoc>,
    #[visit]
    pub imports: Vec<GTImport>,
    #[visit]
    pub aliases: Vec<GTAlias>,
}
