use super::name::GTName;

mod parse;

#[derive(Debug, PartialEq, Clone)]
pub struct GTInlineImport {
    pub path: String,
    pub name: GTName,
}
