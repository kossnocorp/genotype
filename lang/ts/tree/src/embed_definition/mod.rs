use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone)]
pub struct TSEmbedDefinition {
    pub name: TSIdentifier,
    pub embed: GtlEmbed,
}

impl TSEmbedDefinition {
    pub fn new(name: TSIdentifier, embed: GtlEmbed) -> Self {
        TSEmbedDefinition { name, embed }
    }
}
