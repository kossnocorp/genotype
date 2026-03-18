use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct TSObject {
    pub properties: Vec<TSProperty>,
}
