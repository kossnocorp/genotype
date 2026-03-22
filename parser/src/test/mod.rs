use crate::prelude::internal::*;

#[cfg(test)]
pub use indoc::indoc;
#[cfg(test)]
pub use insta::{assert_debug_snapshot, assert_ron_snapshot, assert_snapshot};
#[cfg(test)]
pub use pretty_assertions::{
    assert_eq as assert_equal, assert_ne as assert_not_equal, assert_str_eq as assert_str_equal,
};

mod parser;
pub use parser::*;

pub struct Gt {}

impl Gt {
    pub fn literal_boolean(value: bool) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Boolean(value),
        }
    }

    pub fn literal_integer(value: i64) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Integer(value),
        }
    }

    pub fn literal_float(value: f64) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Float(value),
        }
    }

    pub fn literal_string(value: &str) -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::String(value.into()),
        }
    }

    pub fn literal_null() -> GTLiteral {
        GTLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GTLiteralValue::Null,
        }
    }

    pub fn primitive_boolean() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Boolean,
        }
    }

    pub fn primitive_string() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::String,
        }
    }

    pub fn primitive_number() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Number,
        }
    }

    pub fn primitive_i8() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int8,
        }
    }

    pub fn primitive_i16() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int16,
        }
    }

    pub fn primitive_i32() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int32,
        }
    }

    pub fn primitive_i64() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int64,
        }
    }

    pub fn primitive_i128() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Int128,
        }
    }

    pub fn primitive_isize() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntSize,
        }
    }

    pub fn primitive_u8() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU8,
        }
    }

    pub fn primitive_u16() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU16,
        }
    }

    pub fn primitive_u32() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU32,
        }
    }

    pub fn primitive_u64() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU64,
        }
    }

    pub fn primitive_u128() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntU128,
        }
    }

    pub fn primitive_usize() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::IntUSize,
        }
    }

    pub fn primitive_f32() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Float32,
        }
    }

    pub fn primitive_f64() -> GTPrimitive {
        GTPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GTPrimitiveKind::Float64,
        }
    }

    pub fn identifier(name: &str) -> GTIdentifier {
        GTIdentifier((0, 0).into(), name.into())
    }

    pub fn definition_id(name: &str) -> GTDefinitionId {
        GTDefinitionId("module".into(), name.into())
    }

    pub fn reference_definition_id(name: &str) -> GTReferenceDefinitionId {
        GTReferenceDefinitionId::Resolved(Self::definition_id(name))
    }

    pub fn reference_id() -> GTReferenceId {
        GTReferenceId("module".into(), (0, 0).into())
    }

    pub fn reference(name: &str) -> GTReference {
        GTReference {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            id: Self::reference_id(),
            definition_id: Self::reference_definition_id(name),
            identifier: Self::identifier(name),
        }
    }

    pub fn inline_import(path: &str, name: &str) -> GTInlineImport {
        GTInlineImport {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            path: Self::path(path),
            name: Self::identifier(name),
        }
    }

    pub fn path(path: &str) -> GTPath {
        GTPath::new((0, 0).into(), Self::path_module_id(), path.into())
    }

    pub fn path_module_id() -> GTPathModuleId {
        GTPathModuleId::Resolved("path/to/module".into())
    }

    pub fn object(name: &str, properties: Vec<GTProperty>) -> GTObject {
        GTObject {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), name.into()).into(),
            extensions: vec![],
            properties,
        }
    }

    pub fn property<Type>(name: &str, descriptor: Type) -> GTProperty
    where
        Type: Into<GTDescriptor>,
    {
        GTProperty {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTKey::new((0, 0).into(), name.into()),
            descriptor: descriptor.into(),
            required: true,
        }
    }

    pub fn property_optional<Type>(name: &str, descriptor: Type) -> GTProperty
    where
        Type: Into<GTDescriptor>,
    {
        GTProperty {
            required: false,
            ..Gt::property(name, descriptor)
        }
    }

    pub fn alias<Type>(name: &str, descriptor: Type) -> GTAlias
    where
        Type: Into<GTDescriptor>,
    {
        GTAlias {
            id: GTDefinitionId("module".into(), name.into()),
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), name.into()),
            descriptor: descriptor.into(),
        }
    }

    pub fn some_doc(doc: &str) -> Option<GTDoc> {
        Some(Self::doc(doc))
    }

    pub fn doc(doc: &str) -> GTDoc {
        GTDoc((0, 0).into(), doc.into())
    }

    pub fn descriptor<Type>(descriptor: Type) -> GTDescriptor
    where
        Type: Into<GTDescriptor>,
    {
        descriptor.into()
    }

    pub fn array<Type>(descriptor: Type) -> GTArray
    where
        Type: Into<GTDescriptor>,
    {
        GTArray {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            descriptor: descriptor.into(),
        }
    }

    pub fn tuple(descriptors: Vec<GTDescriptor>) -> GTTuple {
        GTTuple {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            descriptors,
        }
    }

    pub fn any() -> GTAny {
        GTAny {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
        }
    }

    pub fn branded(name: &str, primitive: GTPrimitive) -> GTBranded {
        GTBranded {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            id: Self::definition_id(name),
            name: Self::identifier(name),
            primitive,
        }
    }

    pub fn record<Type>(key: GTRecordKey, descriptor: Type) -> GTRecord
    where
        Type: Into<GTDescriptor>,
    {
        GTRecord {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            key,
            descriptor: descriptor.into(),
        }
    }

    pub fn record_key_string() -> GTRecordKey {
        GTRecordKey::String((0, 0).into())
    }

    pub fn record_key_number() -> GTRecordKey {
        GTRecordKey::Number((0, 0).into())
    }

    pub fn record_key_i8() -> GTRecordKey {
        GTRecordKey::Int8((0, 0).into())
    }

    pub fn record_key_i16() -> GTRecordKey {
        GTRecordKey::Int16((0, 0).into())
    }

    pub fn record_key_i32() -> GTRecordKey {
        GTRecordKey::Int32((0, 0).into())
    }

    pub fn record_key_i64() -> GTRecordKey {
        GTRecordKey::Int64((0, 0).into())
    }

    pub fn record_key_i128() -> GTRecordKey {
        GTRecordKey::Int128((0, 0).into())
    }

    pub fn record_key_isize() -> GTRecordKey {
        GTRecordKey::IntSize((0, 0).into())
    }

    pub fn record_key_u8() -> GTRecordKey {
        GTRecordKey::IntU8((0, 0).into())
    }

    pub fn record_key_u16() -> GTRecordKey {
        GTRecordKey::IntU16((0, 0).into())
    }

    pub fn record_key_u32() -> GTRecordKey {
        GTRecordKey::IntU32((0, 0).into())
    }

    pub fn record_key_u64() -> GTRecordKey {
        GTRecordKey::IntU64((0, 0).into())
    }

    pub fn record_key_u128() -> GTRecordKey {
        GTRecordKey::IntU128((0, 0).into())
    }

    pub fn record_key_usize() -> GTRecordKey {
        GTRecordKey::IntUSize((0, 0).into())
    }

    pub fn record_key_f32() -> GTRecordKey {
        GTRecordKey::Float32((0, 0).into())
    }

    pub fn record_key_f64() -> GTRecordKey {
        GTRecordKey::Float64((0, 0).into())
    }

    pub fn attribute<Type>(name: &str, descriptor: Type) -> GTAttribute
    where
        Type: Into<GTAttributeDescriptor>,
    {
        GTAttribute {
            span: (0, 2).into(),
            name: Self::attribute_name(name),
            descriptor: Some(descriptor.into()),
        }
    }

    pub fn attribute_name(value: &str) -> GTAttributeName {
        GTAttributeName {
            span: (0, 0).into(),
            value: value.into(),
        }
    }

    pub fn attribute_assignment<Type>(value: Type) -> GTAttributeAssignment
    where
        Type: Into<GTAttributeValue>,
    {
        GTAttributeAssignment {
            span: (0, 0).into(),
            value: value.into(),
        }
    }

    pub fn context() -> GTContext {
        GTContext::new("module".into())
    }
}
