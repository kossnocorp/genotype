use crate::prelude::internal::*;

mod resolve;
pub(crate) use resolve::*;
mod render;

/// Prepared recursive alias declarations. Non-recursive aliases have no metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TsEffectAlias {
    pub generic: Option<TsEffectGeneric>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TsEffectInterface {
    Recursive {
        native: Box<TsInterface>,
        annotate_fields: bool,
    },
    Generic {
        definition: Box<TsEffectGeneric>,
        properties: Vec<TsEffectGeneric>,
    },
}

/// Structural types and the schema expression evaluated inside a suspension.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TsEffectGeneric {
    pub structural: TsDescriptor,
    pub codec: TsEffectCodec,
    pub deferred: TsDescriptor,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TsEffectCodec {
    pub decoded: TsDescriptor,
    pub encoded: TsDescriptor,
}

/// Module-scoped preparation, stored separately from the TypeScript syntax tree.
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct TsEffectResolution {
    pub aliases: IndexMap<TsIdentifier, TsEffectAlias>,
    pub interfaces: IndexMap<TsIdentifier, TsEffectInterface>,
    // Suspension depends on the descriptor's resolved reference relationships, so
    // structurally identical descriptors share the same decision at any location.
    pub suspended_descriptors: Vec<TsDescriptor>,
}
