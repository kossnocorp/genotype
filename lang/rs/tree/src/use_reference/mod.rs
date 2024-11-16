use crate::use_name::RSUseName;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum RSUseReference {
    Module,
    Glob,
    Named(Vec<RSUseName>),
}
