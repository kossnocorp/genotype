use crate::prelude::internal::*;

mod parser;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Visitor)]
pub enum GtDescriptor {
    Alias(#[visit] Box<GtAlias>),
    Array(#[visit] Box<GtArray>),
    InlineImport(#[visit] GtInlineImport),
    Literal(#[visit] GtLiteral),
    Object(#[visit] GtObject),
    Primitive(#[visit] GtPrimitive),
    Reference(#[visit] GtReference),
    Tuple(#[visit] GtTuple),
    Union(#[visit] GtUnion),
    Record(#[visit] Box<GtRecord>),
    Any(#[visit] GtAny),
    Branded(#[visit] GtBranded),
}

impl GtDescriptor {
    pub fn span(&self) -> GtSpan {
        match &self {
            GtDescriptor::Alias(alias) => alias.span,
            GtDescriptor::Array(array) => array.span,
            GtDescriptor::InlineImport(inline_import) => inline_import.span,
            GtDescriptor::Literal(literal) => literal.span,
            GtDescriptor::Object(object) => object.span,
            GtDescriptor::Primitive(primitive) => primitive.span,
            GtDescriptor::Reference(reference) => reference.span,
            GtDescriptor::Tuple(tuple) => tuple.span,
            GtDescriptor::Union(union) => union.span,
            GtDescriptor::Record(record) => record.span,
            GtDescriptor::Any(any) => any.span,
            GtDescriptor::Branded(branded) => branded.span,
        }
    }

    pub fn node(&self) -> GtNode {
        match &self {
            GtDescriptor::Alias(_) => GtNode::Alias,
            GtDescriptor::Array(_) => GtNode::Array,
            GtDescriptor::InlineImport(_) => GtNode::InlineImport,
            GtDescriptor::Literal(_) => GtNode::Literal,
            GtDescriptor::Object(_) => GtNode::Object,
            GtDescriptor::Primitive(_) => GtNode::Primitive,
            GtDescriptor::Reference(_) => GtNode::Reference,
            GtDescriptor::Tuple(_) => GtNode::Tuple,
            GtDescriptor::Union(_) => GtNode::Union,
            GtDescriptor::Record(_) => GtNode::Record,
            GtDescriptor::Any(_) => GtNode::Any,
            GtDescriptor::Branded(_) => GtNode::Branded,
        }
    }

    pub fn attributes(&self) -> &Vec<GtAttribute> {
        match &self {
            GtDescriptor::Alias(alias) => &alias.attributes,
            GtDescriptor::Array(array) => &array.attributes,
            GtDescriptor::InlineImport(inline_import) => &inline_import.attributes,
            GtDescriptor::Literal(literal) => &literal.attributes,
            GtDescriptor::Object(object) => &object.attributes,
            GtDescriptor::Primitive(primitive) => &primitive.attributes,
            GtDescriptor::Reference(reference) => &reference.attributes,
            GtDescriptor::Tuple(tuple) => &tuple.attributes,
            GtDescriptor::Union(union) => &union.attributes,
            GtDescriptor::Record(record) => &record.attributes,
            GtDescriptor::Any(any) => &any.attributes,
            GtDescriptor::Branded(branded) => &branded.attributes,
        }
    }
}

//#region Alias

impl From<GtAlias> for GtDescriptor {
    fn from(alias: GtAlias) -> Self {
        GtDescriptor::Alias(Box::new(alias))
    }
}

impl TryFrom<GtDescriptor> for GtAlias {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Alias(alias) => Ok(*alias),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Array

impl From<GtArray> for GtDescriptor {
    fn from(array: GtArray) -> Self {
        GtDescriptor::Array(Box::new(array))
    }
}

impl TryFrom<GtDescriptor> for GtArray {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Array(array) => Ok(*array),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region InlineImport

impl From<GtInlineImport> for GtDescriptor {
    fn from(inline_import: GtInlineImport) -> Self {
        GtDescriptor::InlineImport(inline_import)
    }
}

impl TryFrom<GtDescriptor> for GtInlineImport {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::InlineImport(inline_import) => Ok(inline_import),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Object

impl From<GtObject> for GtDescriptor {
    fn from(object: GtObject) -> Self {
        GtDescriptor::Object(object)
    }
}

impl TryFrom<GtDescriptor> for GtObject {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Object(object) => Ok(object),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Literal

impl From<GtLiteral> for GtDescriptor {
    fn from(literal: GtLiteral) -> Self {
        GtDescriptor::Literal(literal)
    }
}

impl TryFrom<GtDescriptor> for GtLiteral {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Literal(literal) => Ok(literal),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Primitive

impl TryFrom<GtDescriptor> for GtPrimitive {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Primitive(primitive) => Ok(primitive),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Reference

impl From<GtReference> for GtDescriptor {
    fn from(reference: GtReference) -> Self {
        GtDescriptor::Reference(reference)
    }
}

impl TryFrom<GtDescriptor> for GtReference {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Reference(reference) => Ok(reference),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Tuple

impl From<GtTuple> for GtDescriptor {
    fn from(tuple: GtTuple) -> Self {
        GtDescriptor::Tuple(tuple)
    }
}

impl TryFrom<GtDescriptor> for GtTuple {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Tuple(tuple) => Ok(tuple),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Union

impl From<GtUnion> for GtDescriptor {
    fn from(union: GtUnion) -> Self {
        GtDescriptor::Union(union)
    }
}

impl TryFrom<GtDescriptor> for GtUnion {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Union(union) => Ok(union),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Record

impl From<GtRecord> for GtDescriptor {
    fn from(record: GtRecord) -> Self {
        GtDescriptor::Record(Box::new(record))
    }
}

impl TryFrom<GtDescriptor> for GtRecord {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Record(record) => Ok(*record),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Any

impl From<GtAny> for GtDescriptor {
    fn from(any: GtAny) -> Self {
        GtDescriptor::Any(any)
    }
}

impl TryFrom<GtDescriptor> for GtAny {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Any(any) => Ok(any),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion

//#region Branded

impl From<GtBranded> for GtDescriptor {
    fn from(branded: GtBranded) -> Self {
        GtDescriptor::Branded(branded)
    }
}

impl TryFrom<GtDescriptor> for GtBranded {
    type Error = GtParseError;

    fn try_from(descriptor: GtDescriptor) -> Result<Self, GtParseError> {
        match descriptor {
            GtDescriptor::Branded(branded) => Ok(branded),
            _ => Err(GtParseError::UnmatchedDescriptor(
                descriptor.span(),
                descriptor.node(),
            )),
        }
    }
}

//#endregion
