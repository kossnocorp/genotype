use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSModule {
    pub doc: Option<TSDoc>,
    pub imports: Vec<TSImport>,
    pub definitions: Vec<TSDefinition>,
}
