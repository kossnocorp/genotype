use super::{alias::GTAlias, doc::GTDoc, import::GTImport, GTPath};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    pub doc: Option<GTDoc>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
