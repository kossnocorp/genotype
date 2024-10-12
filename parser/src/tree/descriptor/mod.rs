use super::*;

mod parser;

#[derive(Debug, PartialEq, Clone)]
pub enum GTDescriptor {
    Alias(Box<GTAlias>),
    Array(Box<GTArray>),
    InlineImport(GTInlineImport),
    Nullable(Box<GTDescriptor>),
    Object(GTObject),
    Primitive(GTPrimitive),
    Reference(GTReference),
    Tuple(GTTuple),
    Union(GTUnion),
}

impl From<GTObject> for GTDescriptor {
    fn from(object: GTObject) -> Self {
        GTDescriptor::Object(object)
    }
}
