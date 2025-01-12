use crate::*;

mod parse;
mod resolve;

use miette::NamedSource;
pub use parse::GTModuleParse;
pub use resolve::*;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    pub id: GTModuleId,
    pub source_code: NamedSource<String>,
    pub doc: Option<GTDoc>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
