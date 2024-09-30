use crate::{import_glob_alias::TSImportGlobAlias, import_name::TSImportName, name::TSName};

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportReference {
    Default(TSName),
    Glob(TSImportGlobAlias),
    Named(Vec<TSImportName>),
}
