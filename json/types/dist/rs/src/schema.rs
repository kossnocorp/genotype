use literals::literal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjSchemaAny {
    GtjSchemaNull(GtjSchemaNull),
    GtjSchemaBoolean(GtjSchemaBoolean),
    GtjSchemaNumber(GtjSchemaNumber),
    GtjSchemaString(GtjSchemaString),
    GtjSchemaObject(GtjSchemaObject),
    GtjSchemaArray(Box<GtjSchemaArray>),
    GtjSchemaUnion(GtjSchemaUnion),
    GtjSchemaTuple(GtjSchemaTuple),
    GtjSchemaLiteral(GtjSchemaLiteral),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaBase {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaNull {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaNullTypeNull,
}

#[literal("null")]
pub struct GtjSchemaNullTypeNull;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaBoolean {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaBooleanTypeBoolean,
}

#[literal("boolean")]
pub struct GtjSchemaBooleanTypeBoolean;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaNumber {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaNumberTypeNumber,
}

#[literal("number")]
pub struct GtjSchemaNumberTypeNumber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaString {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaStringTypeString,
}

#[literal("string")]
pub struct GtjSchemaStringTypeString;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaObject {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaObjectTypeObject,
    pub properties: BTreeMap<String, GtjSchemaAny>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: Option<bool>,
}

#[literal("object")]
pub struct GtjSchemaObjectTypeObject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaArray {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaArrayTypeArray,
    pub items: GtjSchemaAny,
}

#[literal("array")]
pub struct GtjSchemaArrayTypeArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaUnion {
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "anyOf")]
    pub any_of: Vec<GtjSchemaAny>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaTuple {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: GtjSchemaTupleTypeArray,
    #[serde(rename = "prefixItems")]
    pub prefix_items: Vec<GtjSchemaAny>,
}

#[literal("array")]
pub struct GtjSchemaTupleTypeArray;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GtjSchemaLiteral {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#const: GtjSchemaLiteralConst,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GtjSchemaLiteralConst {
    Null(()),
    Boolean(bool),
    Number(f64),
    String(String),
}
