use super::{import_name::GTImportName, name::GTName};

#[derive(Debug, PartialEq, Clone)]
pub enum GTImportReference {
    Glob,
    Names(Vec<GTImportName>),
    Name(GTName),
}
