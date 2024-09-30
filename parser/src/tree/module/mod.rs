use super::{alias::GTAlias, import::GTImport};

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTModule {
    pub doc: Option<String>,
    pub imports: Vec<GTImport>,
    pub aliases: Vec<GTAlias>,
}
