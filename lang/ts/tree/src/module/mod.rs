use crate::prelude::internal::*;

mod convert;
pub use convert::*;
mod render;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSModule {
    #[visit]
    pub doc: Option<TSDoc>,
    #[visit]
    pub imports: Vec<TSImport>,
    #[visit]
    pub definitions: Vec<TSDefinition>,
}
