use super::{
    alias::GTAlias, array::GTArray, name::GTName, object::GTObject, primitive::GTPrimitive,
    reference::GTReference, tuple::GTTuple,
};

mod parser;

#[derive(Debug, PartialEq, Clone)]
pub enum GTDescriptor {
    Alias(Box<GTAlias>),
    Primitive(GTPrimitive),
    Name(GTName),
    Object(GTObject),
    Array(Box<GTArray>),
    Tuple(GTTuple),
    Reference(GTReference),
    Nullable(Box<GTDescriptor>),
}
