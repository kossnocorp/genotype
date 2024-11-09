use crate::import_name::RSImportName;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSImportReference {
    Module,
    Glob,
    Named(Vec<RSImportName>),
}
