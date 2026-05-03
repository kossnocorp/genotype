use crate::prelude::internal::*;

#[cfg(test)]
pub use indoc::indoc;
#[cfg(test)]
pub use insta::{assert_debug_snapshot, assert_ron_snapshot, assert_snapshot};
#[cfg(test)]
pub use pretty_assertions::{
    assert_eq as assert_equal, assert_ne as assert_not_equal, assert_str_eq as assert_str_equal,
};
#[cfg(test)]
pub use std::fs;

mod parser;
pub use parser::*;

pub struct Gt {}

impl Gt {
    pub fn literal_boolean(value: bool) -> GtLiteral {
        GtLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GtLiteralValue::Boolean(value),
        }
    }

    pub fn literal_integer(value: i64) -> GtLiteral {
        GtLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GtLiteralValue::Integer(value),
        }
    }

    pub fn literal_float(value: f64) -> GtLiteral {
        GtLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GtLiteralValue::Float(value),
        }
    }

    pub fn literal_string(value: &str) -> GtLiteral {
        GtLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GtLiteralValue::String(value.into()),
        }
    }

    pub fn literal_null() -> GtLiteral {
        GtLiteral {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            value: GtLiteralValue::Null,
        }
    }

    pub fn primitive_boolean() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Boolean,
        }
    }

    pub fn primitive_string() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::String,
        }
    }

    pub fn primitive_number() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Number,
        }
    }

    pub fn primitive_i8() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Int8,
        }
    }

    pub fn primitive_i16() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Int16,
        }
    }

    pub fn primitive_i32() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Int32,
        }
    }

    pub fn primitive_i64() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Int64,
        }
    }

    pub fn primitive_i128() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Int128,
        }
    }

    pub fn primitive_isize() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntSize,
        }
    }

    pub fn primitive_u8() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntU8,
        }
    }

    pub fn primitive_u16() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntU16,
        }
    }

    pub fn primitive_u32() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntU32,
        }
    }

    pub fn primitive_u64() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntU64,
        }
    }

    pub fn primitive_u128() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntU128,
        }
    }

    pub fn primitive_usize() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::IntUSize,
        }
    }

    pub fn primitive_f32() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Float32,
        }
    }

    pub fn primitive_f64() -> GtPrimitive {
        GtPrimitive {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            kind: GtPrimitiveKind::Float64,
        }
    }

    pub fn identifier(name: &str) -> GtIdentifier {
        GtIdentifier((0, 0).into(), name.into())
    }

    pub fn definition_id(name: &str) -> GtDefinitionId {
        GtDefinitionId("module".into(), name.into())
    }

    pub fn reference_id<Span>(span: Span) -> GtReferenceId
    where
        Span: Into<GtSpan>,
    {
        GtReferenceId("module".into(), span.into())
    }

    pub fn reference_anon(name: &str) -> GtReference {
        Gt::reference(name, (0, 0))
    }

    pub fn reference<Span>(name: &str, span: Span) -> GtReference
    where
        Span: Into<GtSpan> + Clone,
    {
        GtReference {
            span: span.clone().into(),
            doc: None,
            attributes: vec![],
            id: Self::reference_id(span),
            identifier: Self::identifier(name),
            arguments: vec![],
        }
    }

    pub fn inline_import_anon(path: &str, name: &str) -> GtInlineImport {
        Self::inline_import(path, name, (0, 0))
    }

    pub fn inline_import<Span>(path: &str, name: &str, span: Span) -> GtInlineImport
    where
        Span: Into<GtSpan> + Clone,
    {
        GtInlineImport {
            span: span.clone().into(),
            doc: None,
            attributes: vec![],
            path: Self::path(path, span),
            name: Self::identifier(name),
            arguments: vec![],
        }
    }

    pub fn path_anon(path: &str) -> GtPath {
        Self::path(path, (0, 0))
    }

    pub fn path<Span>(path: &str, span: Span) -> GtPath
    where
        Span: Into<GtSpan> + Clone,
    {
        GtPath::new(span.clone().into(), Self::path_module_id(span), path.into())
    }

    pub fn path_module_id<Span>(span: Span) -> GtPathModuleId
    where
        Span: Into<GtSpan>,
    {
        GtPathModuleId::new(span.into(), "module".into())
    }

    pub fn object(name: &str, properties: Vec<GtProperty>) -> GtObject {
        GtObject {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GtIdentifier::new((0, 0).into(), name.into()).into(),
            extensions: vec![],
            properties,
        }
    }

    pub fn property<Type>(name: &str, descriptor: Type) -> GtProperty
    where
        Type: Into<GtDescriptor>,
    {
        GtProperty {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GtKey::new((0, 0).into(), name.into()),
            descriptor: descriptor.into(),
            required: true,
        }
    }

    pub fn property_optional<Type>(name: &str, descriptor: Type) -> GtProperty
    where
        Type: Into<GtDescriptor>,
    {
        GtProperty {
            required: false,
            ..Gt::property(name, descriptor)
        }
    }

    pub fn alias<Type>(name: &str, descriptor: Type) -> GtAlias
    where
        Type: Into<GtDescriptor>,
    {
        GtAlias {
            id: GtDefinitionId("module".into(), name.into()),
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GtIdentifier::new((0, 0).into(), name.into()),
            generics: vec![],
            descriptor: descriptor.into(),
        }
    }

    pub fn some_doc(doc: &str) -> Option<GtDoc> {
        Some(Self::doc(doc))
    }

    pub fn doc(doc: &str) -> GtDoc {
        GtDoc((0, 0).into(), doc.into())
    }

    pub fn descriptor<Type>(descriptor: Type) -> GtDescriptor
    where
        Type: Into<GtDescriptor>,
    {
        descriptor.into()
    }

    pub fn array<Type>(descriptor: Type) -> GtArray
    where
        Type: Into<GtDescriptor>,
    {
        GtArray {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            descriptor: descriptor.into(),
        }
    }

    pub fn tuple(descriptors: Vec<GtDescriptor>) -> GtTuple {
        GtTuple {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            descriptors,
        }
    }

    pub fn any() -> GtAny {
        GtAny {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
        }
    }

    pub fn branded(name: &str, primitive: GtPrimitive) -> GtBranded {
        GtBranded {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            id: Self::definition_id(name),
            name: Self::identifier(name),
            primitive,
        }
    }

    pub fn record<Type>(key: GtRecordKey, descriptor: Type) -> GtRecord
    where
        Type: Into<GtDescriptor>,
    {
        GtRecord {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            key,
            descriptor: descriptor.into(),
        }
    }

    pub fn record_key_string() -> GtRecordKey {
        GtRecordKey::String((0, 0).into())
    }

    pub fn record_key_number() -> GtRecordKey {
        GtRecordKey::Number((0, 0).into())
    }

    pub fn record_key_i8() -> GtRecordKey {
        GtRecordKey::Int8((0, 0).into())
    }

    pub fn record_key_i16() -> GtRecordKey {
        GtRecordKey::Int16((0, 0).into())
    }

    pub fn record_key_i32() -> GtRecordKey {
        GtRecordKey::Int32((0, 0).into())
    }

    pub fn record_key_i64() -> GtRecordKey {
        GtRecordKey::Int64((0, 0).into())
    }

    pub fn record_key_i128() -> GtRecordKey {
        GtRecordKey::Int128((0, 0).into())
    }

    pub fn record_key_isize() -> GtRecordKey {
        GtRecordKey::IntSize((0, 0).into())
    }

    pub fn record_key_u8() -> GtRecordKey {
        GtRecordKey::IntU8((0, 0).into())
    }

    pub fn record_key_u16() -> GtRecordKey {
        GtRecordKey::IntU16((0, 0).into())
    }

    pub fn record_key_u32() -> GtRecordKey {
        GtRecordKey::IntU32((0, 0).into())
    }

    pub fn record_key_u64() -> GtRecordKey {
        GtRecordKey::IntU64((0, 0).into())
    }

    pub fn record_key_u128() -> GtRecordKey {
        GtRecordKey::IntU128((0, 0).into())
    }

    pub fn record_key_usize() -> GtRecordKey {
        GtRecordKey::IntUSize((0, 0).into())
    }

    pub fn record_key_f32() -> GtRecordKey {
        GtRecordKey::Float32((0, 0).into())
    }

    pub fn record_key_f64() -> GtRecordKey {
        GtRecordKey::Float64((0, 0).into())
    }

    pub fn attribute<Type>(name: &str, descriptor: Type) -> GtAttribute
    where
        Type: Into<GtAttributeDescriptor>,
    {
        GtAttribute {
            span: (0, 2).into(),
            name: Self::attribute_name(name),
            descriptor: Some(descriptor.into()),
        }
    }

    pub fn attribute_flag(name: &str) -> GtAttribute {
        GtAttribute {
            span: (0, 2).into(),
            name: Self::attribute_name(name),
            descriptor: None,
        }
    }

    pub fn attribute_name(value: &str) -> GtAttributeName {
        GtAttributeName {
            span: (0, 0).into(),
            value: value.into(),
        }
    }

    pub fn attribute_assignment<Type>(value: Type) -> GtAttributeAssignment
    where
        Type: Into<GtAttributeValue>,
    {
        GtAttributeAssignment {
            span: (0, 0).into(),
            value: value.into(),
        }
    }

    pub fn context() -> GtContext {
        GtContext::new("module".into())
    }

    pub fn union(descriptors: Vec<GtDescriptor>) -> GtUnion {
        GtUnion {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            descriptors,
        }
    }

    pub fn import(path: &str, reference: GtImportReference) -> GtImport {
        GtImport {
            span: (0, 0).into(),
            path: Self::path_anon(path),
            reference,
        }
    }

    pub fn import_reference_name(name: &str) -> GtImportReference {
        GtImportReference::Name((0, 0).into(), Self::identifier(name))
    }

    pub fn module(imports: Vec<GtImport>, aliases: Vec<GtAlias>) -> GtModule {
        GtModule {
            id: "module".into(),
            doc: None,
            imports,
            aliases,
        }
    }
}

#[macro_export]
macro_rules! attribute_node {
    // key = value
    ($key:ident = $value:expr) => {
        Gt::attribute(
            stringify!($key),
            Gt::attribute_assignment(Gt::literal_string($value)),
        )
    };

    // flag-style: attribute_node!(default)
    ($key:ident) => {
        Gt::attribute_flag(stringify!($key))
    };
}

#[macro_export]
macro_rules! assign {
    ($node:expr $(, $field:ident = $value:expr )* $(,)?) => {{
        let mut strct = $node;
        $(
            strct.$field = $value;
        )*
        strct
    }};
}

#[macro_export]
macro_rules! vec_into {
    ($($item:expr),* $(,)?) => {
        vec![
            $(
                ($item).into()
            ),*
        ]
    };
}
