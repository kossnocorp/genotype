use crate::prelude::internal::*;
pub use crate::prelude::internal::{Traverse, Visitor};

#[visitor(
    RSAlias,
    RSAny,
    RSAttribute,
    RSDefinition,
    RSDependencyIdent,
    RSDescriptor,
    RSDoc,
    RSEnum,
    RSEnumVariant,
    RSEnumVariantDescriptor,
    RSField,
    RSFieldName,
    RSMap,
    RSIdentifier,
    RSInlineUse,
    RSModule,
    RSOption,
    RSPath,
    RSPrimitive,
    RSReference,
    RSStruct,
    RSStructFields,
    RSTuple,
    RSUse,
    RSUseName,
    RSUseReference,
    RSVec
)]
pub struct RSVisitor;
