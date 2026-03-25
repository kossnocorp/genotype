use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsObject {
    #[visit]
    pub properties: Vec<TsProperty>,
}
