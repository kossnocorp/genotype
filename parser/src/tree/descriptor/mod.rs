use super::{
    alias::GTAlias, array::GTArray, inline_import::GTInlineImport, name::GTName, object::GTObject,
    primitive::GTPrimitive, tuple::GTTuple,
};

mod parser;

#[derive(Debug, PartialEq, Clone)]
pub enum GTDescriptor {
    Primitive(GTPrimitive),
    Name(GTName),
    Nullable(Box<GTDescriptor>),
    Object(GTObject),
    Array(Box<GTArray>),
    Tuple(GTTuple),
    Alias(Box<GTAlias>),
    InlineImport(GTInlineImport),
}
