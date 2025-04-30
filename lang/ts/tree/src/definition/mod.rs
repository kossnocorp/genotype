use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone)]
pub enum TSDefinition {
    Alias(TSAlias),
    Interface(TSInterface),
    Branded(TSBranded),
    Embed(TSEmbedDefinition),
}

impl TSDefinition {
    pub fn name(&self) -> TSIdentifier {
        match self {
            TSDefinition::Alias(alias) => alias.name.clone(),
            TSDefinition::Interface(interface) => interface.name.clone(),
            TSDefinition::Branded(branded) => branded.name.clone(),
            TSDefinition::Embed(embed) => embed.name.clone(),
        }
    }
}

impl GtlDefinition for TSDefinition {}

impl From<TSBranded> for TSDefinition {
    fn from(branded: TSBranded) -> Self {
        TSDefinition::Branded(branded)
    }
}

impl From<TSAlias> for TSDefinition {
    fn from(alias: TSAlias) -> Self {
        TSDefinition::Alias(alias)
    }
}

impl From<TSEmbedDefinition> for TSDefinition {
    fn from(embed: TSEmbedDefinition) -> Self {
        TSDefinition::Embed(embed)
    }
}
