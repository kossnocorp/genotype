from typing import Union, TypeAlias, Optional, Literal, List
from genotype import Model


class GtjBase(Model):
    name: Optional[str] = None
    doc: Optional[str] = None


class GtjNull(GtjBase, Model):
    type: Literal["null"]


class GtjBoolean(GtjBase, Model):
    type: Literal["boolean"]


class GtjNumber(GtjBase, Model):
    type: Literal["number"]


class GtjString(GtjBase, Model):
    type: Literal["string"]


class GtjLiteral(GtjBase, Model):
    type: Literal["literal"]
    value: Union[str, float, bool, None]


GtjLiteralType: TypeAlias = Union[Literal["string"], Literal["number"], Literal["boolean"], Literal["null"]]


GtjAny: TypeAlias = Union[GtjNull, GtjBoolean, GtjNumber, GtjString, "GtjObject", "GtjArray", "GtjUnion", "GtjTuple", GtjLiteral]


class GtjObject(GtjBase, Model):
    type: Literal["object"]
    properties: List["GtjProperty"]


class GtjProperty(Model):
    type: Literal["property"]
    name: str
    doc: Optional[str] = None
    descriptor: GtjAny
    required: bool


class GtjArray(GtjBase, Model):
    type: Literal["array"]
    descriptor: GtjAny


class GtjUnion(GtjBase, Model):
    type: Literal["union"]
    descriptors: List[GtjAny]


class GtjTuple(GtjBase, Model):
    type: Literal["tuple"]
    descriptors: List[GtjAny]
