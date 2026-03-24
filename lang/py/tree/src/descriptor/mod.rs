use crate::prelude::internal::*;

mod convert;
mod render;

#[derive(Debug, PartialEq, Clone, Serialize, Visitor)]
pub enum PYDescriptor {
    List(#[visit] Box<PYList>),
    Literal(#[visit] PYLiteral),
    Primitive(#[visit] PYPrimitive),
    Reference(#[visit] PYReference),
    Tuple(#[visit] PYTuple),
    Union(#[visit] PYUnion),
    Dict(#[visit] Box<PYDict>),
    Any(#[visit] PYAny),
}

impl From<PYPrimitive> for PYDescriptor {
    fn from(primitive: PYPrimitive) -> Self {
        PYDescriptor::Primitive(primitive)
    }
}

impl From<PYReference> for PYDescriptor {
    fn from(reference: PYReference) -> Self {
        PYDescriptor::Reference(reference)
    }
}

impl From<PYUnion> for PYDescriptor {
    fn from(union: PYUnion) -> Self {
        PYDescriptor::Union(union)
    }
}

impl From<PYTuple> for PYDescriptor {
    fn from(tuple: PYTuple) -> Self {
        PYDescriptor::Tuple(tuple)
    }
}

impl From<PYList> for PYDescriptor {
    fn from(list: PYList) -> Self {
        PYDescriptor::List(Box::new(list))
    }
}

impl From<PYDict> for PYDescriptor {
    fn from(dict: PYDict) -> Self {
        PYDescriptor::Dict(Box::new(dict))
    }
}

impl From<PYLiteral> for PYDescriptor {
    fn from(literal: PYLiteral) -> Self {
        PYDescriptor::Literal(literal)
    }
}

impl From<PYAny> for PYDescriptor {
    fn from(any: PYAny) -> Self {
        PYDescriptor::Any(any)
    }
}
