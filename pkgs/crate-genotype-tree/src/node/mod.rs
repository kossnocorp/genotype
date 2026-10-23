use crate::prelude::internal::*;

mod alias;
pub use alias::*;

mod any;
pub use any::*;

mod array;
pub use array::*;

mod attribute;
pub use attribute::*;

mod attribute_assignment;
pub use attribute_assignment::*;

mod attribute_descriptor;
pub use attribute_descriptor::*;

mod attribute_key;
pub use attribute_key::*;

mod attribute_name;
pub use attribute_name::*;

mod attribute_property;
pub use attribute_property::*;

mod attribute_value;
pub use attribute_value::*;

mod branded;
pub use branded::*;

mod definition_id;
pub use definition_id::*;

mod descriptor;
pub use descriptor::*;

mod doc;
pub use doc::*;

mod extension;
pub use extension::*;

mod generic_parameter;
pub use generic_parameter::*;

mod generic_argument;
pub use generic_argument::*;

mod identifier;
pub use identifier::*;

mod import;
pub use import::*;

mod import_name;
pub use import_name::*;

mod import_reference;
pub use import_reference::*;

mod inline_import;
pub use inline_import::*;

mod key;
pub use key::*;

mod literal;
pub use literal::*;

mod literal_value;
pub use literal_value::*;

mod module;
pub use module::*;

mod object;
pub use object::*;

mod object_name;
pub use object_name::*;

mod path;
pub use path::*;

mod path_module_id;
pub use path_module_id::*;

mod primitive;
pub use primitive::*;

mod primitive_kind;
pub use primitive_kind::*;

mod property;
pub use property::*;

mod record;
pub use record::*;

mod record_key;
pub use record_key::*;

mod reference;
pub use reference::*;

mod reference_id;
pub use reference_id::*;

mod tuple;
pub use tuple::*;

mod union;
pub use union::*;

/// Genotype node identifier. Used for error reporting and diagnostics.
#[derive(Debug, PartialEq, Clone, Serialize)]
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
    GenericParameter,
    GenericArgument,
    Identifier,
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
            Self::GenericParameter => "generic parameter",
            Self::GenericArgument => "generic argument",
            Self::Identifier => "identifier",
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
