export type GtjSchemaAny = GtjSchemaNull | GtjSchemaBoolean | GtjSchemaNumber | GtjSchemaString | GtjSchemaObject | GtjSchemaArray | GtjSchemaUnion | GtjSchemaTuple | GtjSchemaLiteral;

export interface GtjSchemaBase {
  title?: string | undefined;
  description?: string | undefined;
}

export interface GtjSchemaNull extends GtjSchemaBase {
  type: "null";
}

export interface GtjSchemaBoolean extends GtjSchemaBase {
  type: "boolean";
}

export interface GtjSchemaNumber extends GtjSchemaBase {
  type: "number";
}

export interface GtjSchemaString extends GtjSchemaBase {
  type: "string";
}

export interface GtjSchemaObject extends GtjSchemaBase {
  type: "object";
  properties: Record<string, GtjSchemaAny>;
  required?: Array<string> | undefined;
  additionalProperties?: boolean | undefined;
}

export interface GtjSchemaArray extends GtjSchemaBase {
  type: "array";
  items: GtjSchemaAny;
}

export interface GtjSchemaUnion extends GtjSchemaBase {
  anyOf: Array<GtjSchemaAny>;
}

export interface GtjSchemaTuple extends GtjSchemaBase {
  type: "array";
  prefixItems: Array<GtjSchemaAny>;
}

export interface GtjSchemaLiteral extends GtjSchemaBase {
  const: null | boolean | number | string;
}
