use crate::*;

mod parse;
mod resolve;

pub use parse::GTModuleParse;
pub use resolve::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GTModule {
    /// Module identifier. Used to reference the module in the project.
    pub id: GTModuleId,
    pub doc: Option<GTDoc>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
