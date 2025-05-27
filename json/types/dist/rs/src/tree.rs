use litty::literal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjAny {
    GtjNull(GtjNull),
    GtjBoolean(GtjBoolean),
    GtjNumber(GtjNumber),
    GtjString(GtjString),
    GtjObject(GtjObject),
    GtjArray(Box<GtjArray>),
    GtjUnion(GtjUnion),
    GtjTuple(GtjTuple),
    GtjLiteral(GtjLiteral),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjNull {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjNullTypeNull,
}

#[literal("null")]
pub struct GtjNullTypeNull;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjBoolean {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjBooleanTypeBoolean,
}

#[literal("boolean")]
pub struct GtjBooleanTypeBoolean;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjNumber {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjNumberTypeNumber,
}

#[literal("number")]
pub struct GtjNumberTypeNumber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjString {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjStringTypeString,
}

#[literal("string")]
pub struct GtjStringTypeString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjObjectTypeObject,
    pub properties: Vec<GtjProperty>,
}

#[literal("object")]
pub struct GtjObjectTypeObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjProperty {
    pub r#type: GtjPropertyTypeProperty,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub descriptor: GtjAny,
    pub required: bool,
}

#[literal("property")]
pub struct GtjPropertyTypeProperty;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjArray {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjArrayTypeArray,
    pub descriptor: GtjAny,
}

#[literal("array")]
pub struct GtjArrayTypeArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjUnion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjUnionTypeUnion,
    pub descriptors: Vec<GtjAny>,
}

#[literal("union")]
pub struct GtjUnionTypeUnion;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjTuple {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjTupleTypeTuple,
    pub descriptors: Vec<GtjAny>,
}

#[literal("tuple")]
pub struct GtjTupleTypeTuple;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjLiteral {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<String>,
    pub r#type: GtjLiteralTypeLiteral,
    pub value: GtjLiteralValue,
}

#[literal("literal")]
pub struct GtjLiteralTypeLiteral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjLiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjLiteralType {
    String(GtjLiteralTypeString),
    Number(GtjLiteralTypeNumber),
    Boolean(GtjLiteralTypeBoolean),
    Null(GtjLiteralTypeNull),
}

#[literal("string")]
pub struct GtjLiteralTypeString;

#[literal("number")]
pub struct GtjLiteralTypeNumber;

#[literal("boolean")]
pub struct GtjLiteralTypeBoolean;

#[literal("null")]
pub struct GtjLiteralTypeNull;
