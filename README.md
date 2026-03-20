# Omnis

**Omnis** is a high-performance, Rust-based schema compiler designed to eliminate type drift in polyglot monorepos. Define your types, API contracts, and field documentation **exactly once** in a Git-friendly YAML manifest, and emit strictly typed SDKs and validation schemas for **TypeScript** and **Python**.

> *"Because your Python service shouldn't have to guess what your TypeScript API changed."*

---

## The Core Philosophy

* **Single Source of Truth:** One YAML file drives your Zod schemas (TS), Pydantic models (Py), and OpenAPI documentation.
* **Compile-Time Enforcement:** Changes in the manifest trigger build errors in your consumers if the implementation deviates.
* **Git-Native:** Deterministic code generation. Everything lives in your repo for transparent code reviews and zero-dependency CI/CD.
* **Type Algebra:** First-class support for Unions, Enums, Partials, and complex nested structures.

---

## Features

* **omnis --watch:** Instant feedback loop. Validates your YAML and re-emits code on save.
* **omnis --format:** Opinionated YAML formatter to keep your manifest clean and diff-friendly.
* **AutoDocs:** Automatically maps YAML descriptions to JSDOC (TS), Docstrings (Python), and OpenAPI description fields.
* **Smark Mocks:** Uses example data from your manifest to generate mock payloads for profiling and testing.

## Architecture

Omnis follows a **Parse** -> **Transofrm** -> **Emit** pipeline.
1. **Parser (Rust):** Validates YAML and resolves references into a language-agnostic Itermediate Representation (IR)
2. **Linter:** Checks for circular dependencies, naming collisions, typos, et cetera prior to emission
3. **Emitters:** Pluggable modules that map the IR to idiomatic code (Zod, Pydantic, etc)

## Contributing

Omnis is **FOSS** and built for engineers who value type safety and performance. We welcome contributions, especially for new emitters (Go, Rust, Swift).

1. Clone the repo.
2. Install Rust ```cargo```
3. Run ```cargo test``` to ensure the IR logic holds up
