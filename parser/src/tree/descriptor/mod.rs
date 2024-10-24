use super::*;

mod parser;

#[derive(Debug, PartialEq, Clone)]
pub enum GTDescriptor {
    Alias(Box<GTAlias>),
    Array(Box<GTArray>),
    InlineImport(GTInlineImport),
    Literal(GTLiteral),
    Object(GTObject),
    Primitive(GTPrimitive),
    Reference(GTReference),
    Tuple(GTTuple),
    Union(GTUnion),
    Record(Box<GTRecord>),
}

impl From<GTObject> for GTDescriptor {
    fn from(object: GTObject) -> Self {
        GTDescriptor::Object(object)
    }
}

impl From<GTLiteral> for GTDescriptor {
    fn from(literal: GTLiteral) -> Self {
        GTDescriptor::Literal(literal)
    }
}

impl From<GTReference> for GTDescriptor {
    fn from(reference: GTReference) -> Self {
        GTDescriptor::Reference(reference)
    }
}

impl From<GTIdentifier> for GTDescriptor {
    fn from(identifier: GTIdentifier) -> Self {
        GTDescriptor::Reference(identifier.into())
    }
}

impl From<GTUnion> for GTDescriptor {
    fn from(union: GTUnion) -> Self {
        GTDescriptor::Union(union)
    }
}
