use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PYEmbedDefinition {
    #[visit]
    pub name: PYIdentifier,
    pub embed: GtlEmbed,
}

impl PYEmbedDefinition {
    pub fn new(name: PYIdentifier, embed: GtlEmbed) -> Self {
        PYEmbedDefinition { name, embed }
    }
}
