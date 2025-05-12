export type GtjAny = GtjNull | GtjBoolean | GtjNumber | GtjString | GtjObject | GtjArray | GtjUnion | GtjTuple | GtjLiteral;

export interface GtjBase {
  name?: string | undefined;
  doc?: string | undefined;
}

export interface GtjNull extends GtjBase {
  type: "null";
}

export interface GtjBoolean extends GtjBase {
  type: "boolean";
}

export interface GtjNumber extends GtjBase {
  type: "number";
}

export interface GtjString extends GtjBase {
  type: "string";
}

export interface GtjObject extends GtjBase {
  type: "object";
  properties: Array<GtjProperty>;
}

export interface GtjProperty {
  type: "property";
  name: string;
  doc?: string | undefined;
  descriptor: GtjAny;
  required: boolean;
}

export interface GtjArray extends GtjBase {
  type: "array";
  descriptor: GtjAny;
}

export interface GtjUnion extends GtjBase {
  type: "union";
  descriptors: Array<GtjAny>;
}

export interface GtjTuple extends GtjBase {
  type: "tuple";
  descriptors: Array<GtjAny>;
}

export interface GtjLiteral extends GtjBase {
  type: "literal";
  value: string | number | boolean | null;
}

export type GtjLiteralType = "string" | "number" | "boolean" | "null";
