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
    Any(GTAny),
    Branded(GTBranded),
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

impl From<GTTuple> for GTDescriptor {
    fn from(tuple: GTTuple) -> Self {
        GTDescriptor::Tuple(tuple)
    }
}
impl From<GTUnion> for GTDescriptor {
    fn from(union: GTUnion) -> Self {
        GTDescriptor::Union(union)
    }
}

impl From<GTRecord> for GTDescriptor {
    fn from(record: GTRecord) -> Self {
        GTDescriptor::Record(Box::new(record))
    }
}

impl From<GTAny> for GTDescriptor {
    fn from(any: GTAny) -> Self {
        GTDescriptor::Any(any)
    }
}

impl From<GTBranded> for GTDescriptor {
    fn from(branded: GTBranded) -> Self {
        GTDescriptor::Branded(branded)
    }
}
