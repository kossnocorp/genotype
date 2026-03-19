use crate::prelude::internal::*;

mod parser;

#[derive(Debug, PartialEq, Clone, Serialize)]
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

impl GTDescriptor {
    fn span(&self) -> GTSpan {
        match &self {
            GTDescriptor::Alias(alias) => alias.span,
            GTDescriptor::Array(array) => array.span,
            GTDescriptor::InlineImport(inline_import) => inline_import.span,
            GTDescriptor::Literal(literal) => literal.span,
            GTDescriptor::Object(object) => object.span,
            GTDescriptor::Primitive(primitive) => primitive.span,
            GTDescriptor::Reference(reference) => reference.span,
            GTDescriptor::Tuple(tuple) => tuple.span,
            GTDescriptor::Union(union) => union.span,
            GTDescriptor::Record(record) => record.span,
            GTDescriptor::Any(any) => any.span(),
            GTDescriptor::Branded(branded) => branded.span,
        }
    }

    fn node(&self) -> GTNode {
        match &self {
            GTDescriptor::Alias(_) => GTNode::Alias,
            GTDescriptor::Array(_) => GTNode::Array,
            GTDescriptor::InlineImport(_) => GTNode::InlineImport,
            GTDescriptor::Literal(_) => GTNode::Literal,
            GTDescriptor::Object(_) => GTNode::Object,
            GTDescriptor::Primitive(_) => GTNode::Primitive,
            GTDescriptor::Reference(_) => GTNode::Reference,
            GTDescriptor::Tuple(_) => GTNode::Tuple,
            GTDescriptor::Union(_) => GTNode::Union,
            GTDescriptor::Record(_) => GTNode::Record,
            GTDescriptor::Any(_) => GTNode::Branded,
            GTDescriptor::Branded(_) => GTNode::Branded,
        }
    }
}

//#region Alias

impl From<GTAlias> for GTDescriptor {
    fn from(alias: GTAlias) -> Self {
        GTDescriptor::Alias(Box::new(alias))
    }
}

impl TryFrom<GTDescriptor> for GTAlias {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Alias(alias) => Ok(*alias),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Array

impl From<GTArray> for GTDescriptor {
    fn from(array: GTArray) -> Self {
        GTDescriptor::Array(Box::new(array))
    }
}

impl TryFrom<GTDescriptor> for GTArray {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Array(array) => Ok(*array),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region InlineImport

impl TryFrom<GTDescriptor> for GTInlineImport {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::InlineImport(inline_import) => Ok(inline_import),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Object

impl From<GTObject> for GTDescriptor {
    fn from(object: GTObject) -> Self {
        GTDescriptor::Object(object)
    }
}

impl TryFrom<GTDescriptor> for GTObject {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Object(object) => Ok(object),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Literal

impl From<GTLiteral> for GTDescriptor {
    fn from(literal: GTLiteral) -> Self {
        GTDescriptor::Literal(literal)
    }
}

impl TryFrom<GTDescriptor> for GTLiteral {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Literal(literal) => Ok(literal),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Primitive

impl TryFrom<GTDescriptor> for GTPrimitive {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Primitive(primitive) => Ok(primitive),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Reference

impl From<GTReference> for GTDescriptor {
    fn from(reference: GTReference) -> Self {
        GTDescriptor::Reference(reference)
    }
}

impl TryFrom<GTDescriptor> for GTReference {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Reference(reference) => Ok(reference),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Tuple

impl From<GTTuple> for GTDescriptor {
    fn from(tuple: GTTuple) -> Self {
        GTDescriptor::Tuple(tuple)
    }
}

impl TryFrom<GTDescriptor> for GTTuple {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Tuple(tuple) => Ok(tuple),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Union

impl From<GTUnion> for GTDescriptor {
    fn from(union: GTUnion) -> Self {
        GTDescriptor::Union(union)
    }
}

impl TryFrom<GTDescriptor> for GTUnion {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Union(union) => Ok(union),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Record

impl From<GTRecord> for GTDescriptor {
    fn from(record: GTRecord) -> Self {
        GTDescriptor::Record(Box::new(record))
    }
}

impl TryFrom<GTDescriptor> for GTRecord {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Record(record) => Ok(*record),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Any

impl From<GTAny> for GTDescriptor {
    fn from(any: GTAny) -> Self {
        GTDescriptor::Any(any)
    }
}

impl TryFrom<GTDescriptor> for GTAny {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Any(any) => Ok(any),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Branded

impl From<GTBranded> for GTDescriptor {
    fn from(branded: GTBranded) -> Self {
        GTDescriptor::Branded(branded)
    }
}

impl TryFrom<GTDescriptor> for GTBranded {
    type Error = GTParseError;

    fn try_from(descriptor: GTDescriptor) -> Result<Self, GTParseError> {
        match descriptor {
            GTDescriptor::Branded(branded) => Ok(branded),
            _ => Err(GTParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion
