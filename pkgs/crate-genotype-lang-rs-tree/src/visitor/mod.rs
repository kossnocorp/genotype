use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, Visitor};

#[visitor(
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
)]
pub struct RsVisitor;
