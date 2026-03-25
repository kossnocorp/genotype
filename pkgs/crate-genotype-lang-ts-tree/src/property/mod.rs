use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsProperty {
    #[visit]
    pub doc: Option<TsDoc>,
    #[visit]
    pub name: TsKey,
    #[visit]
    pub descriptor: TsDescriptor,
    pub required: bool,
}
