use super::{alias::GTAlias, doc::GTDoc, import::GTImport};

mod parse;
use miette::NamedSource;
pub use parse::GTModuleParse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    pub source_code: NamedSource<String>,
    pub doc: Option<GTDoc>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
