use super::import_name::GTImportName;

#[derive(Debug, PartialEq, Clone)]
pub enum GTImportReference {
    Glob,
    Names(Vec<GTImportName>),
    Name(String),
}