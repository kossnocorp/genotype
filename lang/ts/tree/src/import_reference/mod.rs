use crate::{import_glob_alias::TSImportGlobAlias, import_name::TSImportName};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportReference {
    Default(String),
    Glob(TSImportGlobAlias),
    Named(Vec<TSImportName>),
}
