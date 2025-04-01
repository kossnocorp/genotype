export type JsonAny = JsonNull | JsonBoolean | JsonNumber | JsonString | JsonArray | JsonObject | JsonUnion | JsonLiteral | JsonTuple;

export interface JsonBase {
  name?: string | undefined;
  doc?: string | undefined;
}

export interface JsonNull extends JsonBase {
  kind: "null";
}

export interface JsonBoolean extends JsonBase {
  kind: "boolean";
}

export interface JsonNumber extends JsonBase {
  kind: "number";
}

export interface JsonString extends JsonBase {
  kind: "string";
}

export interface JsonArray extends JsonBase {
  kind: "array";
  descriptor: JsonAny;
}

export interface JsonObject extends JsonBase {
  kind: "object";
  properties: Array<JsonProperty>;
}

export interface JsonProperty {
  kind: "property";
  name: string;
  doc?: string | undefined;
  descriptor: JsonAny;
  required?: boolean | undefined;
}

export interface JsonUnion extends JsonBase {
  kind: "union";
  descriptors: Array<JsonAny>;
}

export interface JsonTuple extends JsonBase {
  kind: "tuple";
  descriptors: Array<JsonAny>;
}

export interface JsonLiteral extends JsonBase {
  kind: "literal";
  value: string | number | boolean | null;
}

export type JsonLiteralKind = "string" | "number" | "boolean" | "null";
