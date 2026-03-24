use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, Visitor};

#[visitor(
    PYAlias,
    PYAny,
    PYClass,
    PYDefinition,
    PYDependencyIdent,
    PYDescriptor,
    PYDict,
    PYDictKey,
    PYDoc,
    PYEmbedDefinition,
    PYExtension,
    PYIdentifier,
    PYImport,
    PYImportName,
    PYImportReference,
    PYKey,
    PYList,
    PYLiteral,
    PYModule,
    PYPath,
    PYPrimitive,
    PYProperty,
    PYReference,
    PYTuple,
    PYUnion,
    PYNewtype
)]
pub struct PYVisitor;
