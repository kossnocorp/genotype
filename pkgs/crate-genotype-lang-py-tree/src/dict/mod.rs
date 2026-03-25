use crate::prelude::internal::*;

mod context;
mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyDict {
    #[visit]
    pub key: PyDictKey,
    #[visit]
    pub descriptor: PyDescriptor,
}
