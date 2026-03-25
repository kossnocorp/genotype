use crate::prelude::internal::*;

pub use genotype_visitor_macro::{Visitor, visitor};

pub trait Traverse<V: ?Sized> {
    fn traverse(&mut self, visitor: &mut V);
}

impl<V: ?Sized, T> Traverse<V> for Option<T>
where
    T: Traverse<V>,
{
    fn traverse(&mut self, visitor: &mut V) {
        if let Some(inner) = self {
            inner.traverse(visitor);
        }
    }
}

impl<V: ?Sized, T> Traverse<V> for Vec<T>
where
    T: Traverse<V>,
{
    fn traverse(&mut self, visitor: &mut V) {
        for item in self {
            item.traverse(visitor);
        }
    }
}

impl<V: ?Sized, T> Traverse<V> for Box<T>
where
    T: Traverse<V>,
{
    fn traverse(&mut self, visitor: &mut V) {
        self.as_mut().traverse(visitor);
    }
}

#[visitor(
    GtAlias,
    GtArray,
    GtAttribute,
    GtAttributeAssignment,
    GtAttributeDescriptor,
    GtAttributeKey,
    GtAttributeName,
    GtAttributeProperty,
    GtAttributeValue,
    GtDescriptor,
    GtDoc,
    GtExtension,
    GtIdentifier,
    GtImport,
    GtImportName,
    GtImportReference,
    GtInlineImport,
    GtKey,
    GtLiteral,
    GtModule,
    GtObject,
    GtObjectName,
    GtPath,
    GtPrimitive,
    GtProperty,
    GtReference,
    GtRecord,
    GtRecordKey,
    GtTuple,
    GtUnion,
    GtAny,
    GtBranded
)]
pub struct GtVisitor;
