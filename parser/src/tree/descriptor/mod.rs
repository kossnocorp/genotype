use super::{
    alias::GTAlias, array::GTArray, inline_import::GTInlineImport, object::GTObject,
    primitive::GTPrimitive, reference::GTReference, tuple::GTTuple,
};

mod parser;

#[derive(Debug, PartialEq, Clone)]
pub enum GTDescriptor {
    Primitive(GTPrimitive),
    Reference(GTReference),
    Nullable(Box<GTDescriptor>),
    Object(GTObject),
    Array(Box<GTArray>),
    Tuple(GTTuple),
    Alias(Box<GTAlias>),
    InlineImport(GTInlineImport),
}
