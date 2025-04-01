from typing import Optional, Literal
from genotype import Model


class GtjBase(Model):
    name: Optional[str] = None
    doc: Optional[str] = None


class GtjNull(GtjBase, Model):
    kind: Literal["null"]


class GtjBoolean(GtjBase, Model):
    kind: Literal["boolean"]


class GtjNumber(GtjBase, Model):
    kind: Literal["number"]


class GtjString(GtjBase, Model):
    kind: Literal["string"]


class GtjLiteral(GtjBase, Model):
    kind: Literal["literal"]
    value: str | float | bool | None


type GtjLiteralKind = Literal["string"] | Literal["number"] | Literal["boolean"] | Literal["null"]


type GtjAny = GtjNull | GtjBoolean | GtjNumber | GtjString | GtjArray | GtjObject | GtjUnion | GtjLiteral | GtjTuple


class GtjArray(GtjBase, Model):
    kind: Literal["array"]
    descriptor: GtjAny


class GtjObject(GtjBase, Model):
    kind: Literal["object"]
    properties: list[GtjProperty]


class GtjProperty(Model):
    kind: Literal["property"]
    name: str
    doc: Optional[str] = None
    descriptor: GtjAny
    required: Optional[bool] = None


class GtjUnion(GtjBase, Model):
    kind: Literal["union"]
    descriptors: list[GtjAny]


class GtjTuple(GtjBase, Model):
    kind: Literal["tuple"]
    descriptors: list[GtjAny]
