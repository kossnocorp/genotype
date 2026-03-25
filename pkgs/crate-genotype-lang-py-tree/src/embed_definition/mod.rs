use crate::prelude::internal::*;

mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub struct PyEmbedDefinition {
    #[visit]
    pub name: PyIdentifier,
    pub embed: GtlEmbed,
}

impl PyEmbedDefinition {
    pub fn new(name: PyIdentifier, embed: GtlEmbed) -> Self {
        PyEmbedDefinition { name, embed }
    }
}
