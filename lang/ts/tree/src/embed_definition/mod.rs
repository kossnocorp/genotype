use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct TSEmbedDefinition {
    #[visit]
    pub name: TSIdentifier,
    pub embed: GtlEmbed,
}

impl TSEmbedDefinition {
    pub fn new(name: TSIdentifier, embed: GtlEmbed) -> Self {
        TSEmbedDefinition { name, embed }
    }
}
