use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSObject {
    pub properties: Vec<TSProperty>,
}
