use crate::{import_name::TSImportName, name::TSName};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportReference {
    Default(TSName),
    Glob(TSName),
    Named(Vec<TSImportName>),
}
