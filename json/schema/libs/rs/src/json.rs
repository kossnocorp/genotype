use serde::{Deserialize, Serialize};
use literals::literal;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonAny {
    JsonNull(JsonNull),
    JsonBoolean(JsonBoolean),
    JsonNumber(JsonNumber),
    JsonString(JsonString),
    JsonArray(JsonArray),
    JsonObject(JsonObject),
    JsonUnion(JsonUnion),
    JsonLiteral(JsonLiteral),
    JsonTuple(JsonTuple),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonBase {
    pub name: Option<String>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonNull {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonNullKindNull,
}

#[literal("null")]
pub struct JsonNullKindNull;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonBoolean {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonBooleanKindBoolean,
}

#[literal("boolean")]
pub struct JsonBooleanKindBoolean;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonNumber {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonNumberKindNumber,
}

#[literal("number")]
pub struct JsonNumberKindNumber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonString {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonStringKindString,
}

#[literal("string")]
pub struct JsonStringKindString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonArray {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonArrayKindArray,
    pub descriptor: JsonAny,
}

#[literal("array")]
pub struct JsonArrayKindArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonObject {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonObjectKindObject,
    pub properties: Vec<JsonProperty>,
}

#[literal("object")]
pub struct JsonObjectKindObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonProperty {
    pub kind: JsonPropertyKindProperty,
    pub name: String,
    pub doc: Option<String>,
    pub descriptor: JsonAny,
    pub required: Option<bool>,
}

#[literal("property")]
pub struct JsonPropertyKindProperty;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonUnion {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonUnionKindUnion,
    pub descriptors: Vec<JsonAny>,
}

#[literal("union")]
pub struct JsonUnionKindUnion;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonTuple {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonTupleKindTuple,
    pub descriptors: Vec<JsonAny>,
}

#[literal("tuple")]
pub struct JsonTupleKindTuple;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonLiteral {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonLiteralKindLiteral,
    pub value: JsonLiteralValue,
}

#[literal("literal")]
pub struct JsonLiteralKindLiteral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonLiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonLiteralKind {
    String(JsonLiteralKindString),
    Number(JsonLiteralKindNumber),
    Boolean(JsonLiteralKindBoolean),
    Null(JsonLiteralKindNull),
}

#[literal("string")]
pub struct JsonLiteralKindString;

#[literal("number")]
pub struct JsonLiteralKindNumber;

#[literal("boolean")]
pub struct JsonLiteralKindBoolean;

#[literal("null")]
pub struct JsonLiteralKindNull;
