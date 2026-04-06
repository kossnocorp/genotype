use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, TraverseMut, Visitor};

#[visitor(
    nodes(
        TsAlias,
        TsAny,
        TsArray,
        TsBranded,
        TsDefinition,
        TsDependencyIdent,
        TsDescriptor,
        TsDoc,
        TsEmbedDefinition,
        TsExtension,
        TsIdentifier,
        TsImport,
        TsImportName,
        TsImportReference,
        TsInlineImport,
        TsInterface,
        TsIntersection,
        TsKey,
        TsLiteral,
        TsModule,
        TsObject,
        TsPath,
        TsPrimitive,
        TsProperty,
        TsRecord,
        TsRecordKey,
        TsReference,
        TsTuple,
        TsUnion
    ),
    mut_trait = TsVisitorMut
)]
pub struct TsVisitor;
