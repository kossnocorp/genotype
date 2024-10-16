use crate::import_name::PYImportName;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYImportReference {
    Default(String),
    Glob(String),
    Named(Vec<PYImportName>),
}
