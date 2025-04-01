use serde::{Deserialize, Serialize};
use literals::literal;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchema {
    JsonSchemaNull(JsonSchemaNull),
    JsonSchemaBoolean(JsonSchemaBoolean),
    JsonSchemaNumber(JsonSchemaNumber),
    JsonSchemaString(JsonSchemaString),
    JsonSchemaArray(JsonSchemaArray),
    JsonSchemaObject(JsonSchemaObject),
    JsonSchemaUnion(JsonSchemaUnion),
    JsonSchemaLiteral(JsonSchemaLiteral),
    JsonSchemaTuple(JsonSchemaTuple),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaBase {
    pub name: Option<String>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaNull {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaNullKindNull,
}

#[literal("null")]
pub struct JsonSchemaNullKindNull;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaBoolean {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaBooleanKindBoolean,
}

#[literal("boolean")]
pub struct JsonSchemaBooleanKindBoolean;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaNumber {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaNumberKindNumber,
}

#[literal("number")]
pub struct JsonSchemaNumberKindNumber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaString {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaStringKindString,
}

#[literal("string")]
pub struct JsonSchemaStringKindString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaArray {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaArrayKindArray,
    pub descriptor: JsonSchema,
}

#[literal("array")]
pub struct JsonSchemaArrayKindArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaObject {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaObjectKindObject,
    pub properties: Vec<JsonSchemaProperty>,
}

#[literal("object")]
pub struct JsonSchemaObjectKindObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaProperty {
    pub kind: JsonSchemaPropertyKindProperty,
    pub name: String,
    pub doc: Option<String>,
    pub descriptor: JsonSchema,
    pub required: Option<bool>,
}

#[literal("property")]
pub struct JsonSchemaPropertyKindProperty;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaUnion {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaUnionKindUnion,
    pub descriptors: Vec<JsonSchema>,
}

#[literal("union")]
pub struct JsonSchemaUnionKindUnion;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaTuple {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaTupleKindTuple,
    pub descriptors: Vec<JsonSchema>,
}

#[literal("tuple")]
pub struct JsonSchemaTupleKindTuple;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchemaLiteral {
    pub name: Option<String>,
    pub doc: Option<String>,
    pub kind: JsonSchemaLiteralKindLiteral,
    pub value: JsonSchemaLiteralValue,
}

#[literal("literal")]
pub struct JsonSchemaLiteralKindLiteral;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaLiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null(()),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonSchemaLiteralKind {
    String(JsonSchemaLiteralKindString),
    Number(JsonSchemaLiteralKindNumber),
    Boolean(JsonSchemaLiteralKindBoolean),
    Null(JsonSchemaLiteralKindNull),
}

#[literal("string")]
pub struct JsonSchemaLiteralKindString;

#[literal("number")]
pub struct JsonSchemaLiteralKindNumber;

#[literal("boolean")]
pub struct JsonSchemaLiteralKindBoolean;

#[literal("null")]
pub struct JsonSchemaLiteralKindNull;
