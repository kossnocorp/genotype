use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum GTNode {
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
    Literal,
    Module,
    Object,
    ObjectName,
    Path,
    Primitive,
    Property,
    Record,
    RecordKey,
}

impl GTNode {
    pub fn name(&self) -> &'static str {
        match self {
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
            Self::Literal => "literal",
            Self::Module => "module",
            Self::Object => "object",
            Self::ObjectName => "object name",
            Self::Path => "path",
            Self::Primitive => "primitive",
            Self::Property => "property",
            Self::Record => "record",
            Self::RecordKey => "record key",
        }
    }
}

impl Display for GTNode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.name())
    }
}
