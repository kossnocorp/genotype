export type JsonSchema = JsonSchemaNull | JsonSchemaBoolean | JsonSchemaNumber | JsonSchemaString | JsonSchemaArray | JsonSchemaObject | JsonSchemaUnion | JsonSchemaLiteral | JsonSchemaTuple;

export interface JsonSchemaBase {
  name?: string | undefined;
  doc?: string | undefined;
}

export interface JsonSchemaNull extends JsonSchemaBase {
  kind: "null";
}

export interface JsonSchemaBoolean extends JsonSchemaBase {
  kind: "boolean";
}

export interface JsonSchemaNumber extends JsonSchemaBase {
  kind: "number";
}

export interface JsonSchemaString extends JsonSchemaBase {
  kind: "string";
}

export interface JsonSchemaArray extends JsonSchemaBase {
  kind: "array";
  descriptor: JsonSchema;
}

export interface JsonSchemaObject extends JsonSchemaBase {
  kind: "object";
  properties: Array<JsonSchemaProperty>;
}

export interface JsonSchemaProperty {
  kind: "property";
  name: string;
  doc?: string | undefined;
  descriptor: JsonSchema;
  required?: boolean | undefined;
}

export interface JsonSchemaUnion extends JsonSchemaBase {
  kind: "union";
  descriptors: Array<JsonSchema>;
}

export interface JsonSchemaTuple extends JsonSchemaBase {
  kind: "tuple";
  descriptors: Array<JsonSchema>;
}

export interface JsonSchemaLiteral extends JsonSchemaBase {
  kind: "literal";
  value: string | number | boolean | null;
}

export type JsonSchemaLiteralKind = "string" | "number" | "boolean" | "null";
