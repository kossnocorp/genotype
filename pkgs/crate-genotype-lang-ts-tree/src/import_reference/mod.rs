use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TSImportReference {
    Default(Arc<str>),
    Glob(Arc<str>),
    Named(#[visit] Vec<TSImportName>),
}

impl From<&str> for TSImportReference {
    fn from(str: &str) -> Self {
        TSImportReference::Named(vec![str.into()])
    }
}
