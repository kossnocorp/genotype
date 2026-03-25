use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsInterface {
    #[visit]
    pub doc: Option<TsDoc>,
    #[visit]
    pub name: TsIdentifier,
    #[visit]
    pub extensions: Vec<TsExtension>,
    #[visit]
    pub properties: Vec<TsProperty>,
}
