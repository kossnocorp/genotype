from typing import Union, TypeAlias, Optional, Literal, Dict, List
from genotype import Model


class GtjSchemaBase(Model):
    title: Optional[str] = None
    description: Optional[str] = None


class GtjSchemaNull(GtjSchemaBase, Model):
    type: Literal["null"]


class GtjSchemaBoolean(GtjSchemaBase, Model):
    type: Literal["boolean"]


class GtjSchemaNumber(GtjSchemaBase, Model):
    type: Literal["number"]


class GtjSchemaString(GtjSchemaBase, Model):
    type: Literal["string"]


class GtjSchemaLiteral(GtjSchemaBase, Model):
    const: Union[None, bool, float, str]


GtjSchemaAny: TypeAlias = Union[GtjSchemaNull, GtjSchemaBoolean, GtjSchemaNumber, GtjSchemaString, "GtjSchemaObject", "GtjSchemaArray", "GtjSchemaUnion", "GtjSchemaTuple", GtjSchemaLiteral]


class GtjSchemaObject(GtjSchemaBase, Model):
    type: Literal["object"]
    properties: Dict[str, GtjSchemaAny]
    required: Optional[List[str]] = None
    additional_properties: Optional[bool] = None


class GtjSchemaArray(GtjSchemaBase, Model):
    type: Literal["array"]
    items: GtjSchemaAny


class GtjSchemaUnion(GtjSchemaBase, Model):
    any_of: List[GtjSchemaAny]


class GtjSchemaTuple(GtjSchemaBase, Model):
    type: Literal["array"]
    prefix_items: List[GtjSchemaAny]
