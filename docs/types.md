# Omnis Type System (v0.1.0)

Omnis uses a **Hardware-Aware** type system. While high-level languages (TypeScript/Python) may flatten these for ergonomics, the underlying IR preserves precision for systems languages (Go/Rust/C++).

## The Omnis Philosophy: No `any`

Omnis does not support an `any` or `unknown` type. All data must be structured.

- **If it's third-party JSON:** Model the fields you need.
- **If it's a dynamic lookup:** Use `map(string, T)`.
- **If it's binary:** Use `bytes`.
  Type safety is a non-negotiable requirement for cross-language code generation.

## 1. Primitives

| Omnis Type | TS           | Python     | Go / Rust           | Intent                 |
| :--------- | :----------- | :--------- | :------------------ | :--------------------- |
| `string`   | `string`     | `str`      | `string` / `String` | UTF-8 Text             |
| `int32`    | `number`     | `int`      | `int32` / `i32`     | Standard integer       |
| `int64`    | `bigint`     | `int`      | `int64` / `i64`     | Large IDs / Snowflakes |
| `float64`  | `number`     | `float`    | `float64` / `f64`   | Precision decimals     |
| `boolean`  | `boolean`    | `bool`     | `bool`              | True / False           |
| `datetime` | `Date`       | `datetime` | `time.Time`         | ISO-8601               |
| `uuid`     | `string`     | `UUID`     | `[16]byte` / `Uuid` | Unique Identifier      |
| `bytes`    | `Uint8Array` | `bytes`    | `[]byte`            | Binary data            |

## 2. Collections

Omnis supports functional wrapper syntax for collections to ensure clear nesting.

- **`list(T)`**: An ordered sequence. (TS: `T[]`, Go: `[]T`, Py: `list[T]`)
- **`map(K, V)`**: A key-value lookup. (TS: `Record<K, V>`, Go: `map[K]V`, Py: `dict[K, V]`)
- **`set(T)`**: A collection of unique elements. (TS: `Set<T>`, Go: `map[T]struct{}`, Py: `set[T]`)
- **`tuple(T, U, ...)`**: A fixed-length, ordered sequence. (TS: `[T, U]`, Py: `tuple[T, U]`, Rust: `(T, U)`)

## 3. The Nullability Contract

Omnis enforces a strict distinction between **Data Presence** and **Data Absence**.

- **`T | null`**: **Explicit Nothing.** The field exists but has no value. (TS: `.nullable()`)
- **`T | undefined`**: **Optional Entry.** The field itself may be omitted from the payload. (TS: `.optional()`)

## 4. Type Algebra (Utility Types)

- **`$extends: $Model`**: Inherit all fields from a base model.
- **`$pick: { model: $M, fields: [...] }`**: Create a model with only specific fields.
- **`$omit: { model: $M, fields: [...] }`**: Create a model excluding specific fields.
- **`$partial: $Model`**: Convert all fields in a model to `T | undefined`.

## 5. Generics (MVP 2)

Templates are defined using the `Name(T)` syntax and "hydrated" upon invocation.

```yaml
# Definition
Envelope(T):
  status: int32
  payload: $T

# Usage
UserResponse:
  $model: $Envelope($User)
```
