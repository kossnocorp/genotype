use crate::{import_name::RSImportName, RSIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSImportReference {
    Default(Option<RSIdentifier>),
    Glob,
    Named(Vec<RSImportName>),
}
