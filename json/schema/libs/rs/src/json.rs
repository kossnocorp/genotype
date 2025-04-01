use serde::{Deserialize, Serialize};
use literals::literal;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjAny {
    GtjNull(GtjNull),
    GtjBoolean(GtjBoolean),
    GtjNumber(GtjNumber),
    GtjString(GtjString),
    GtjArray(GtjArray),
    GtjObject(GtjObject),
    GtjUnion(GtjUnion),
    GtjLiteral(GtjLiteral),
    GtjTuple(GtjTuple),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjBase {
    pub name: Option<String>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjNull {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjNullKindNull,
}

#[literal("null")]
pub struct GtjNullKindNull;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjBoolean {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjBooleanKindBoolean,
}

#[literal("boolean")]
pub struct GtjBooleanKindBoolean;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjNumber {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjNumberKindNumber,
}

#[literal("number")]
pub struct GtjNumberKindNumber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjString {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjStringKindString,
}

#[literal("string")]
pub struct GtjStringKindString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjArray {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjArrayKindArray,
    pub descriptor: GtjAny,
}

#[literal("array")]
pub struct GtjArrayKindArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjObject {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjObjectKindObject,
    pub properties: Vec<GtjProperty>,
}

#[literal("object")]
pub struct GtjObjectKindObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjProperty {
    pub kind: GtjPropertyKindProperty,
    pub name: String,
    pub doc: Option<String>,
    pub descriptor: GtjAny,
    pub required: Option<bool>,
}

#[literal("property")]
pub struct GtjPropertyKindProperty;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjUnion {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjUnionKindUnion,
    pub descriptors: Vec<GtjAny>,
}

#[literal("union")]
pub struct GtjUnionKindUnion;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjTuple {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjTupleKindTuple,
    pub descriptors: Vec<GtjAny>,
}

#[literal("tuple")]
pub struct GtjTupleKindTuple;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjLiteral {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: GtjLiteralKindLiteral,
    pub value: GtjLiteralValue,
}

#[literal("literal")]
pub struct GtjLiteralKindLiteral;

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
pub enum GtjLiteralKind {
    String(GtjLiteralKindString),
    Number(GtjLiteralKindNumber),
    Boolean(GtjLiteralKindBoolean),
    Null(GtjLiteralKindNull),
}

#[literal("string")]
pub struct GtjLiteralKindString;

#[literal("number")]
pub struct GtjLiteralKindNumber;

#[literal("boolean")]
pub struct GtjLiteralKindBoolean;

#[literal("null")]
pub struct GtjLiteralKindNull;
