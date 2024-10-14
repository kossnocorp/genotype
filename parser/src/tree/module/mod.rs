use crate::GTSourceCode;

use super::{alias::GTAlias, doc::GTDoc, import::GTImport};

mod parse;
pub use parse::GTModuleParse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    pub source_code: GTSourceCode,
    pub doc: Option<GTDoc>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
