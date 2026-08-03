use crate::prelude::internal::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub struct GtAttributeProperty {
    pub span: GtSpan,
    #[visit]
    pub name: GtAttributeKey,
    pub value: GtAttributeValue,
}
