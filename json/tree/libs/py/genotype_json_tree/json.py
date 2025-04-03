from typing import Union, TypeAlias, Optional, Literal, List
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
    value: Union[str, float, bool, None]


GtjLiteralKind: TypeAlias = Union[Literal["string"], Literal["number"], Literal["boolean"], Literal["null"]]


GtjAny: TypeAlias = Union[GtjNull, GtjBoolean, GtjNumber, GtjString, "GtjArray", "GtjObject", "GtjUnion", GtjLiteral, "GtjTuple"]


class GtjArray(GtjBase, Model):
    kind: Literal["array"]
    descriptor: GtjAny


class GtjObject(GtjBase, Model):
    kind: Literal["object"]
    properties: List["GtjProperty"]


class GtjProperty(Model):
    kind: Literal["property"]
    name: str
    doc: Optional[str] = None
    descriptor: GtjAny
    required: bool


class GtjUnion(GtjBase, Model):
    kind: Literal["union"]
    descriptors: List[GtjAny]


class GtjTuple(GtjBase, Model):
    kind: Literal["tuple"]
    descriptors: List[GtjAny]
