use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, Visitor};

#[visitor(
    TSAlias,
    TSAny,
    TSArray,
    TSBranded,
    TSDefinition,
    TSDescriptor,
    TSDoc,
    TSEmbedDefinition,
    TSExtension,
    TSIdentifier,
    TSImport,
    TSImportName,
    TSImportReference,
    TSInlineImport,
    TSInterface,
    TSIntersection,
    TSKey,
    TSLiteral,
    TSModule,
    TSObject,
    TSPath,
    TSPrimitive,
    TSProperty,
    TSRecord,
    TSRecordKey,
    TSReference,
    TSTuple,
    TSUnion
)]
pub struct TSVisitor;
