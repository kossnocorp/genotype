use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, TraverseMut, Visitor};

#[visitor(
    nodes(
        RsAlias,
        RsAny,
        RsAttribute,
        RsDefinition,
        RsDependencyIdent,
        RsDescriptor,
        RsDoc,
        RsEnum,
        RsEnumVariant,
        RsEnumVariantDescriptor,
        RsField,
        RsFieldName,
        RsMap,
        RsIdentifier,
        RsInlineUse,
        RsModule,
        RsOption,
        RsPath,
        RsPrimitive,
        RsReference,
        RsStruct,
        RsStructFields,
        RsTuple,
        RsUse,
        RsUseName,
        RsUseReference,
        RsVec
    ),
    mut_trait = RsVisitorMut
)]
pub struct RsVisitor;
