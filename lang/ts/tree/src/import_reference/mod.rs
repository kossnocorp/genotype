use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportReference {
    Default(String),
    Glob(String),
    Named(Vec<TSImportName>),
}
