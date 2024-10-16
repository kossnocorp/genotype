use crate::{import_name::PYImportName, PYIdentifier};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum PYImportReference {
    Default(Option<PYIdentifier>),
    Glob,
    Named(Vec<PYImportName>),
}
