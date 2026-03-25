use crate::prelude::internal::*;

/// Genotype node identifier. Used for error reporting and diagnostics.
#[derive(Debug, PartialEq)]
pub enum GtNode {
    Any,
    Alias,
    Array,
    Attribute,
    AttributeAssignment,
    AttributeDescriptor,
    AttributeProperty,
    AttributeValue,
    Descriptor,
    Extension,
    Import,
    InlineImport,
    Literal,
    Module,
    Object,
    ObjectName,
    Path,
    Primitive,
    Reference,
    Tuple,
    Union,
    Property,
    Record,
    RecordKey,
    Branded,
}

impl GtNode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Alias => "alias",
            Self::Attribute => "attribute",
            Self::AttributeAssignment => "attribute assignment",
            Self::AttributeDescriptor => "attribute descriptor",
            Self::AttributeProperty => "attribute property",
            Self::AttributeValue => "attribute value",
            Self::Array => "array",
            Self::Descriptor => "descriptor",
            Self::Extension => "extension",
            Self::Import => "import",
            Self::InlineImport => "inline import",
            Self::Literal => "literal",
            Self::Module => "module",
            Self::Object => "object",
            Self::ObjectName => "object name",
            Self::Path => "path",
            Self::Primitive => "primitive",
            Self::Reference => "reference",
            Self::Tuple => "tuple",
            Self::Union => "union",
            Self::Property => "property",
            Self::Record => "record",
            Self::RecordKey => "record key",
            Self::Branded => "branded primitive",
        }
    }
}

impl Display for GtNode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.name())
    }
}
