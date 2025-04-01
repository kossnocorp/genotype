from typing import Optional, Literal
from genotype import Model


class JsonBase(Model):
    name: Optional[str] = None
    doc: Optional[str] = None


class JsonNull(JsonBase, Model):
    kind: Literal["null"]


class JsonBoolean(JsonBase, Model):
    kind: Literal["boolean"]


class JsonNumber(JsonBase, Model):
    kind: Literal["number"]


class JsonString(JsonBase, Model):
    kind: Literal["string"]


class JsonLiteral(JsonBase, Model):
    kind: Literal["literal"]
    value: str | float | bool | None


type JsonLiteralKind = Literal["string"] | Literal["number"] | Literal["boolean"] | Literal["null"]


type JsonAny = JsonNull | JsonBoolean | JsonNumber | JsonString | JsonArray | JsonObject | JsonUnion | JsonLiteral | JsonTuple


class JsonArray(JsonBase, Model):
    kind: Literal["array"]
    descriptor: JsonAny


class JsonObject(JsonBase, Model):
    kind: Literal["object"]
    properties: list[JsonProperty]


class JsonProperty(Model):
    kind: Literal["property"]
    name: str
    doc: Optional[str] = None
    descriptor: JsonAny
    required: Optional[bool] = None


class JsonUnion(JsonBase, Model):
    kind: Literal["union"]
    descriptors: list[JsonAny]


class JsonTuple(JsonBase, Model):
    kind: Literal["tuple"]
    descriptors: list[JsonAny]
