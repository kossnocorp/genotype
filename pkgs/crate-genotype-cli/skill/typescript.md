# TypeScript Target Overview

Genotype generates idiomatic TypeScript code that can be used directly in the application or published as a package.

This overview shows how Genotype types translate to TypeScript, target-specific features and configuration options.

See the [Quick Genotype Language Tour](language.md) for the language overview and [TypeScript Configuration](typescript-configuration.md) for detailed configuration reference.

## Translation

The following examples give an overview of how Genotype translates into TypeScript.

**Note:**

Object examples show [Interfaces and
Aliases](typescript-configuration.md#tsprefer---interfaces-or-type-aliases); other
examples use Types. All include [Zod
output](typescript-configuration.md#tsmode---generation-mode). Feature examples omit
imports; Zod examples use `z` from [Zod](https://www.npmjs.com/package/zod). The [Complete
Module](#complete-module) includes imports.

### Complete Module

This source combines an object, a literal field, an optional field, a record, and `any`:

```type
Book: {
  kind: "book",
  displayTitle: string,
  subtitle?: string,
  ratings: { []: int },
  extra: any,
}
```

The generated module includes its definitions and any required imports:

**Interfaces**

```ts
export interface Book {
  kind: "book";
  displayTitle: string;
  subtitle?: string | undefined;
  ratings: Record<string, number>;
  extra: any;
}
```

**Aliases**

```ts
export type Book = {
  kind: "book";
  displayTitle: string;
  subtitle?: string | undefined;
  ratings: Record<string, number>;
  extra: any;
};
```

**Zod**

```ts
import { z } from "zod";

export const Book = z.object({
  kind: z.literal("book"),
  displayTitle: z.string(),
  subtitle: z.union([z.string(), z.undefined()]).optional(),
  ratings: z.record(z.string(), z.number()),
  extra: z.any(),
});

export type Book = z.infer<typeof Book>;
```

### Unions

Genotype unions translate directly to TypeScript union types.

```type
Value: string | int
```

**Types**

```ts
export type Value = string | number;
```

**Zod**

```ts
export const Value = z.union([z.string(), z.number()]);

export type Value = z.infer<typeof Value>;
```

### Primitives

#### Numeric Types

##### `number`

Genotype `number` translates directly to the TypeScript `number` type.

```type
Amount: number
```

**Types**

```ts
export type Amount = number;
```

**Zod**

```ts
export const Amount = z.number();

export type Amount = z.infer<typeof Amount>;
```

##### `int`

Genotype `int` translates into the TypeScript umbrella `number` type.

```type
Count: int
```

**Types**

```ts
export type Count = number;
```

**Zod**

```ts
export const Count = z.number();

export type Count = z.infer<typeof Count>;
```

##### `float`

Genotype `float` translates into the TypeScript umbrella `number` type.

```type
Ratio: float
```

**Types**

```ts
export type Ratio = number;
```

**Zod**

```ts
export const Ratio = z.number();

export type Ratio = z.infer<typeof Ratio>;
```

##### Sized Numeric Types

Sized Genotype numeric types translate into the TypeScript umbrella `number` type, except for `i128` and `u128`, which translate into `bigint`.

```type
SmallCount: i16
PreciseRatio: f32
LargeCount: i128
```

**Types**

```ts
export type SmallCount = number;

export type PreciseRatio = number;

export type LargeCount = bigint;
```

**Zod**

```ts
export const SmallCount = z.number();

export type SmallCount = z.infer<typeof SmallCount>;

export const PreciseRatio = z.number();

export type PreciseRatio = z.infer<typeof PreciseRatio>;

export const LargeCount = z.bigint();

export type LargeCount = z.infer<typeof LargeCount>;
```

#### Booleans

Genotype `boolean` translates directly to the TypeScript `boolean` type.

```type
Ready: boolean
```

**Types**

```ts
export type Ready = boolean;
```

**Zod**

```ts
export const Ready = z.boolean();

export type Ready = z.infer<typeof Ready>;
```

#### Strings

Genotype `string` translates directly to the TypeScript `string` type.

```type
Title: string
```

**Types**

```ts
export type Title = string;
```

**Zod**

```ts
export const Title = z.string();

export type Title = z.infer<typeof Title>;
```

#### Literal Types

Genotype literal types translate directly to TypeScript literal types.

```type
Category: "fiction"
```

**Types**

```ts
export type Category = "fiction";
```

**Zod**

```ts
export const Category = z.literal("fiction");

export type Category = z.infer<typeof Category>;
```

#### Null

Genotype `null` translates directly to the TypeScript `null` type.

```type
Empty: null
```

**Types**

```ts
export type Empty = null;
```

**Zod**

```ts
export const Empty = z.literal(null);

export type Empty = z.infer<typeof Empty>;
```

#### Branded Primitives

Genotype branded primitives translate into branded TypeScript types. Their runtime values remain unchanged.

```type
BookId: @string
```

**Types**

```ts
export type BookId = string & { [bookIdBrand]: true };
declare const bookIdBrand: unique symbol;
```

**Zod**

```ts
export const BookId = z.string().brand<"BookId">();

export type BookId = z.infer<typeof BookId>;
```

### Composite Types

#### Objects

Genotype objects translate into TypeScript interfaces or type aliases. Choose the output style with [ts.prefer](typescript-configuration.md#tsprefer---interfaces-or-type-aliases).

```type
Book: { title: string }
```

**Interfaces**

```ts
export interface Book {
  title: string;
}
```

**Aliases**

```ts
export type Book = {
  title: string;
};
```

**Zod**

```ts
export const Book = z.object({
  title: z.string(),
});

export type Book = z.infer<typeof Book>;
```

##### Optional Object Fields

Optional Genotype object fields translate into optional TypeScript properties.

```type
Draft: { subtitle?: string }
```

**Interfaces**

```ts
export interface Draft {
  subtitle?: string | undefined;
}
```

**Aliases**

```ts
export type Draft = {
  subtitle?: string | undefined;
};
```

**Zod**

```ts
export const Draft = z.object({
  subtitle: z.union([z.string(), z.undefined()]).optional(),
});

export type Draft = z.infer<typeof Draft>;
```

##### Object Extensions

Genotype object extensions translate into interface inheritance or type intersections.

```type
Named: { name: string }
NamedBook: { ...Named, pages: int }
```

**Interfaces**

```ts
export interface Named {
  name: string;
}

export interface NamedBook extends Named {
  pages: number;
}
```

**Aliases**

```ts
export type Named = {
  name: string;
};

export type NamedBook = Named & {
  pages: number;
};
```

**Zod**

```ts
export const Named = z.object({
  name: z.string(),
});

export type Named = z.infer<typeof Named>;

export const NamedBook = Named.extend({
  pages: z.number(),
});

export type NamedBook = z.infer<typeof NamedBook>;
```

#### Arrays

Genotype arrays translate into TypeScript `Array<T>` types.

```type
Titles: [string]
```

**Types**

```ts
export type Titles = Array<string>;
```

**Zod**

```ts
export const Titles = z.array(z.string());

export type Titles = z.infer<typeof Titles>;
```

#### Tuples

Genotype tuples translate directly to TypeScript tuple types.

```type
Point: (float, float)
```

**Types**

```ts
export type Point = [number, number];
```

**Zod**

```ts
export const Point = z.tuple([z.number(), z.number()]);

export type Point = z.infer<typeof Point>;
```

#### Records

Genotype records translate into TypeScript `Record<K, V>` types. An omitted key type means string keys.

```type
Scores: { []: int }
```

**Types**

```ts
export type Scores = Record<string, number>;
```

**Zod**

```ts
export const Scores = z.record(z.string(), z.number());

export type Scores = z.infer<typeof Scores>;
```

### Special Data Types

#### Any Type

Genotype `any` translates directly to the TypeScript `any` type.

```type
Payload: any
```

**Types**

```ts
export type Payload = any;
```

**Zod**

```ts
export const Payload = z.any();

export type Payload = z.infer<typeof Payload>;
```

### Generic Types

Genotype generic types translate into TypeScript generic types. Zod output uses functions accepting schemas, e.g., `Envelope(Book)`.

```type
Envelope<Body>: { body: Body }
```

**Interfaces**

```ts
export interface Envelope<Body> {
  body: Body;
}
```

**Aliases**

```ts
export type Envelope<Body> = {
  body: Body;
};
```

**Zod**

```ts
export const Envelope = <Body extends z.ZodTypeAny>(Body: Body) =>
  z.object({
    body: Body,
  });

export type Envelope<Body extends z.ZodTypeAny> = z.infer<ReturnType<typeof Envelope<Body>>>;
```

### Recursive Types

Genotype recursive types translate directly to TypeScript recursive types. Zod output uses getters or lazy schemas.

```type
LinkedNode: { value: string, next?: LinkedNode }
```

**Interfaces**

```ts
export interface LinkedNode {
  value: string;
  next?: LinkedNode | undefined;
}
```

**Aliases**

```ts
export type LinkedNode = {
  value: string;
  next?: LinkedNode | undefined;
};
```

**Zod**

```ts
export const LinkedNode = z.object({
  value: z.string(),
  get next() {
    return z.union([LinkedNode, z.undefined()]).optional();
  },
});

export type LinkedNode = z.infer<typeof LinkedNode>;
```

### Annotations

Annotations intended for other targets, such as Rust enum variant names, don't affect TypeScript output.

## Configuration

Use `[ts]` in `genotype.toml` to configure TypeScript. See the [TypeScript Configuration Reference](typescript-configuration.md) for more details.

See [Genotype Configuration](configuration.md) for global settings and [Common Target Options](configuration.md#common-target-options).
