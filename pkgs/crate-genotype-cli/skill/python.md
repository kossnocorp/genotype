# Python Target Overview

Genotype generates idiomatic Python code that can be used directly in the application or published as a package.

This overview shows how Genotype types translate to Python, target-specific features and configuration options.

See the [Quick Genotype Language Tour](language.md) for the language overview and [Python Configuration](python-configuration.md) for detailed configuration reference.

## Translation

The following examples give an overview of how Genotype translates into Python.

**Note:**

Latest and Legacy tabs show differences between [Python
versions](python-configuration.md#pyversion---python-version). Examples without tabs
apply to both. Feature examples omit imports. Genotype includes the required standard library,
[Pydantic](https://pypi.org/project/pydantic/), and [Genotype
runtime](https://pypi.org/project/genotype-runtime/) imports. The [complete
module](#complete-module) below shows them together.

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

The generated module includes its type definitions and imports:

**Latest**

```python
from __future__ import annotations

from typing import Literal, Optional, Any
from pydantic import Field
from genotype import Model

class Book(Model):
    kind: Literal["book"]
    display_title: str = Field(alias="displayTitle")
    subtitle: Optional[str] = None
    ratings: dict[str, int]
    extra: Any
```

**Legacy**

```python
from __future__ import annotations

from typing import Literal, Optional, Dict, Any
from pydantic import Field
from genotype import Model

class Book(Model):
    kind: Literal["book"]
    display_title: str = Field(alias="displayTitle")
    subtitle: Optional[str] = None
    ratings: Dict[str, int]
    extra: Any
```

### Unions

Genotype unions translate into Python union types. Legacy output uses `Union[...]`.

```type
Value: string | int
```

**Latest**

```python
type Value = str | int
```

**Legacy**

```python
Value = Union[str, int]
```

### Primitives

#### Numeric Types

##### `number`

Genotype `number` translates into the Python `float` type.

```type
Amount: number
```

**Latest**

```python
type Amount = float
```

**Legacy**

```python
Amount = float
```

##### `int`

Genotype `int` translates directly to the Python `int` type.

```type
Count: int
```

**Latest**

```python
type Count = int
```

**Legacy**

```python
Count = int
```

##### `float`

Genotype `float` translates directly to the Python `float` type.

```type
Ratio: float
```

**Latest**

```python
type Ratio = float
```

**Legacy**

```python
Ratio = float
```

##### Sized Numeric Types

Sized Genotype integers translate into Python `int`, and sized floating-point types translate into `float`.

```type
SmallCount: i16
PreciseRatio: f32
LargeCount: i128
```

**Latest**

```python
type SmallCount = int

type PreciseRatio = float

type LargeCount = int
```

**Legacy**

```python
SmallCount = int

PreciseRatio = float

LargeCount = int
```

#### Booleans

Genotype `boolean` translates into the Python `bool` type.

```type
Ready: boolean
```

**Latest**

```python
type Ready = bool
```

**Legacy**

```python
Ready = bool
```

#### Strings

Genotype `string` translates into the Python `str` type.

```type
Title: string
```

**Latest**

```python
type Title = str
```

**Legacy**

```python
Title = str
```

#### Literal Types

Genotype literal types translate into Python `Literal` types, preserving their exact values.

```type
Category: "fiction"
```

**Latest**

```python
type Category = Literal["fiction"]
```

**Legacy**

```python
Category = Literal["fiction"]
```

#### Null

Genotype `null` translates into Python `Literal[None]`.

```type
Empty: null
```

**Latest**

```python
type Empty = Literal[None]
```

**Legacy**

```python
Empty = Literal[None]
```

#### Branded Primitives

Genotype branded primitives translate into Python `NewType` definitions. Type checkers distinguish them; runtime values retain the underlying primitive.

```type
BookId: @string
```

```python
BookId = NewType("BookId", str)
```

### Composite Types

#### Objects

Genotype objects translate into classes extending the [Genotype runtime](https://pypi.org/project/genotype-runtime/)'s [Pydantic](https://pypi.org/project/pydantic/)-based `Model`. Nested objects become separate model classes.

```type
Book: { title: string }
```

```python
class Book(Model):
    title: str
```

Use `Book.model_validate(data)` to validate incoming data and `book.model_dump()` to serialize it. Field names use `snake_case`, with aliases preserving their original names in serialized data.

##### Optional Object Fields

Optional Genotype object fields translate into Python `Optional[T]` fields, defaulting to `None`.

```type
Draft: { subtitle?: string }
```

```python
class Draft(Model):
    subtitle: Optional[str] = None
```

##### Object Extensions

Genotype object extensions translate into Python model inheritance.

```type
Named: { name: string }
NamedBook: { ...Named, pages: int }
```

```python
class Named(Model):
    name: str

class NamedBook(Named, Model):
    pages: int
```

#### Arrays

Genotype arrays translate into Python `list[T]` types. Legacy output uses `List[T]`.

```type
Titles: [string]
```

**Latest**

```python
type Titles = list[str]
```

**Legacy**

```python
Titles = List[str]
```

#### Tuples

Genotype tuples translate into Python `tuple` types. Legacy output uses `Tuple[...]`.

```type
Point: (float, float)
```

**Latest**

```python
type Point = tuple[float, float]
```

**Legacy**

```python
Point = Tuple[float, float]
```

#### Records

Genotype records translate into Python `dict[K, V]` types. Legacy output uses `Dict[K, V]`. An omitted key type means string keys.

```type
Scores: { []: int }
```

**Latest**

```python
type Scores = dict[str, int]
```

**Legacy**

```python
Scores = Dict[str, int]
```

### Special Data Types

#### Any Type

Genotype `any` translates into Python `typing.Any`.

```type
Payload: any
```

**Latest**

```python
type Payload = Any
```

**Legacy**

```python
Payload = Any
```

### Generic Types

Genotype generic types translate into Python generic models or aliases. You can specialize this model as `Envelope[Book]`.

```type
Envelope<Body>: { body: Body }
```

**Latest**

```python
class Envelope[Body](Model):
    body: Body
```

**Legacy**

```python
Body = TypeVar("Body")

class Envelope(Model, Generic[Body]):
    body: Body
```

### Recursive Types

Genotype recursive types translate into Python recursive types with forward references where needed.

```type
LinkedNode: { value: string, next?: LinkedNode }
```

```python
class LinkedNode(Model):
    value: str
    next: Optional[LinkedNode] = None
```

### Annotations

The `discriminator` annotation adds metadata to a union's JSON Schema:

```type
#[discriminator = "status"]
Result: Success | Failure

Success: { status: "ok", body: string }
Failure: { status: "error", message: string }
```

The generated union includes Pydantic field metadata identifying `status` as the discriminator. This adds schema metadata; it doesn't configure Pydantic's discriminated-union validation.

## Configuration

Use `[py]` in `genotype.toml` to configure Python. See the [Python Configuration Reference](python-configuration.md) for more details.

See [Genotype Configuration](configuration.md) for global settings and [Common Target Options](configuration.md#common-target-options).
