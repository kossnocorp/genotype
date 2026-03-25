use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TsEmbedDefinition {
    #[visit]
    pub name: TsIdentifier,
    pub embed: GtlEmbed,
}

impl TsEmbedDefinition {
    pub fn new(name: TsIdentifier, embed: GtlEmbed) -> Self {
        TsEmbedDefinition { name, embed }
    }
}
