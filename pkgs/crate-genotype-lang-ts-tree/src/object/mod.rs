use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSObject {
    #[visit]
    pub properties: Vec<TSProperty>,
}
