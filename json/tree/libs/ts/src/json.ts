export type GtjAny = GtjNull | GtjBoolean | GtjNumber | GtjString | GtjArray | GtjObject | GtjUnion | GtjLiteral | GtjTuple;

export interface GtjBase {
  name?: string | undefined;
  doc?: string | undefined;
}

export interface GtjNull extends GtjBase {
  kind: "null";
}

export interface GtjBoolean extends GtjBase {
  kind: "boolean";
}

export interface GtjNumber extends GtjBase {
  kind: "number";
}

export interface GtjString extends GtjBase {
  kind: "string";
}

export interface GtjArray extends GtjBase {
  kind: "array";
  descriptor: GtjAny;
}

export interface GtjObject extends GtjBase {
  kind: "object";
  properties: Array<GtjProperty>;
}

export interface GtjProperty {
  kind: "property";
  name: string;
  doc?: string | undefined;
  descriptor: GtjAny;
  required: boolean;
}

export interface GtjUnion extends GtjBase {
  kind: "union";
  descriptors: Array<GtjAny>;
}

export interface GtjTuple extends GtjBase {
  kind: "tuple";
  descriptors: Array<GtjAny>;
}

export interface GtjLiteral extends GtjBase {
  kind: "literal";
  value: string | number | boolean | null;
}

export type GtjLiteralKind = "string" | "number" | "boolean" | "null";
