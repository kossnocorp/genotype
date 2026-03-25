use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum TsDefinition {
    Alias(#[visit] TsAlias),
    Interface(#[visit] TsInterface),
    Branded(#[visit] TsBranded),
    Embed(#[visit] TsEmbedDefinition),
}

impl TsDefinition {
    pub fn name(&self) -> TsIdentifier {
        match self {
            TsDefinition::Alias(alias) => alias.name.clone(),
            TsDefinition::Interface(interface) => interface.name.clone(),
            TsDefinition::Branded(branded) => branded.name.clone(),
            TsDefinition::Embed(embed) => embed.name.clone(),
        }
    }
}

impl GtlDefinition for TsDefinition {}

impl From<TsBranded> for TsDefinition {
    fn from(branded: TsBranded) -> Self {
        TsDefinition::Branded(branded)
    }
}

impl From<TsAlias> for TsDefinition {
    fn from(alias: TsAlias) -> Self {
        TsDefinition::Alias(alias)
    }
}

impl From<TsEmbedDefinition> for TsDefinition {
    fn from(embed: TsEmbedDefinition) -> Self {
        TsDefinition::Embed(embed)
    }
}
