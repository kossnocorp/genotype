from typing import Optional, Literal
from genotype import Model


class JsonSchemaBase(Model):
    name: Optional[str] = None
    doc: Optional[str] = None


class JsonSchemaNull(JsonSchemaBase, Model):
    kind: Literal["null"]


class JsonSchemaBoolean(JsonSchemaBase, Model):
    kind: Literal["boolean"]


class JsonSchemaNumber(JsonSchemaBase, Model):
    kind: Literal["number"]


class JsonSchemaString(JsonSchemaBase, Model):
    kind: Literal["string"]


class JsonSchemaLiteral(JsonSchemaBase, Model):
    kind: Literal["literal"]
    value: str | float | bool | None


type JsonSchemaLiteralKind = Literal["string"] | Literal["number"] | Literal["boolean"] | Literal["null"]


type JsonSchema = JsonSchemaNull | JsonSchemaBoolean | JsonSchemaNumber | JsonSchemaString | JsonSchemaArray | JsonSchemaObject | JsonSchemaUnion | JsonSchemaLiteral | JsonSchemaTuple


class JsonSchemaArray(JsonSchemaBase, Model):
    kind: Literal["array"]
    descriptor: JsonSchema


class JsonSchemaObject(JsonSchemaBase, Model):
    kind: Literal["object"]
    properties: list[JsonSchemaProperty]


class JsonSchemaProperty(Model):
    kind: Literal["property"]
    name: str
    doc: Optional[str] = None
    descriptor: JsonSchema
    required: Optional[bool] = None


class JsonSchemaUnion(JsonSchemaBase, Model):
    kind: Literal["union"]
    descriptors: list[JsonSchema]


class JsonSchemaTuple(JsonSchemaBase, Model):
    kind: Literal["tuple"]
    descriptors: list[JsonSchema]
