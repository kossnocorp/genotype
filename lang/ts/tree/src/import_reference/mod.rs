use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportReference {
    Default(String),
    Glob(String),
    Named(Vec<TSImportName>),
}

impl From<&str> for TSImportReference {
    fn from(str: &str) -> Self {
        TSImportReference::Named(vec![str.into()])
    }
}
