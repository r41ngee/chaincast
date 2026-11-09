# chaincast

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](#development)
[![no dependencies](https://img.shields.io/badge/dependencies-none-brightgreen.svg)](Cargo.toml)

A tiny Rust macro for chained `From` conversions.

`chaincast!` turns a multistep conversion into a single readable line:

```rust
let c: C = chaincast!(a; B, C);
```

instead of a nest of calls you have to read inside-out:

```rust
let c: C = C::from(B::from(a));
```

No dependencies, no runtime cost — it expands to plain `From::from` calls.

## Why

Conversion chains are common in Rust: a value arrives as `String`, gets turned
into `OsString`, then into `PathBuf`, or a new type is unwrapped layer by layer.
Writing `From::from(From::from(From::from(x)))` is noisy and reads backwards.

`chaincast!` keeps the steps in the order they happen, names every intermediate
type, and stays a normal expression, so it can be used anywhere a value is
expected — with `?`, in a `match` arm, in a `return`, or as a tail expression.

## Installation

The crate is **not published on crates.io yet**, so depend on it by git or by path:

```toml
[dependencies]
chaincast = { git = "https://github.com/r41ngee/chaincast" }
```

Local checkout:

```toml
[dependencies]
chaincast = { path = "../chaincast" }
```

Once published, `cargo add chaincast` will work.

## Usage

```rust
use chaincast::chaincast;

struct A;
struct B;
struct C;

impl From<A> for B {
    fn from(_: A) -> B {
        B
    }
}

impl From<B> for C {
    fn from(_: B) -> C {
        C
    }
}

fn main() {
    let a = A;
    let _c: C = chaincast!(a; B, C);
}
```

The macro is exported at the crate root, so `chaincast::chaincast!(...)` works
without a `use` as well.

## Syntax

```rust
chaincast!(expression; Type1, Type2, ..., TypeN);
```

| Part                | Meaning                                                 |
|---------------------|---------------------------------------------------------|
| `expression`        | The value to start from. Any expression.                |
| `;`                 | Required separator between the value and the type list. |
| `Type1, ..., TypeN` | The types the value is converted *through*, in order.   |

The chain ends at `TypeN`; the whole macro evaluates to that value.

## Expansion

```rust
chaincast!(x; B, C)
```

expands to:

```rust
{
    let __inner_result = x;
    let __inner_result = <B>::from(__inner_result);
    let __inner_result = <C>::from(__inner_result);
    __inner_result
}
```

Each step only needs `From` to be implemented for the types you name:

| Step            | Required bound                                         |
|-----------------|--------------------------------------------------------|
| `x → Type1`     | `Type1: From<Input>`, where `Input` is the type of `x` |
| `Type1 → Type2` | `Type2: From<Type1>`                                   |

The temporary bindings live in their own block and are shadowed on every step,
so nothing leaks into the surrounding scope.

## Notes and constraints

- **The list is types, not values or expressions.** Paths and generics are
  fine (`std::path::PathBuf`, `Vec<u8>`, `T`).
- **At least one type is needed** to be useful. Zero types are accepted and
  simply return the input unchanged: `chaincast!(x;)`.
- **Trailing comma is optional.** `chaincast!(a; B, C,)` works too, which is
  handy when a long chain is split across lines.
- **`From` only.** Every step is a plain `From::from` call, so the macro only
  covers infallible conversions. For fallible ones, use `TryFrom`/`TryInto`
  directly.
- **Type inference works like `From::from`.** If a target type is concrete the
  result type is known; if you end on a generic parameter, annotate the binding
  as usual.

## Examples

Numeric widening:

```rust
let n = chaincast!(1u8; u16, u32); // u32
```

Building a path from a string:

```rust
use std::ffi::OsString;
use std::path::PathBuf;

let p = chaincast!(String::from("/tmp"); OsString, PathBuf);
```

Ending on a generic type:

```rust
fn widen<T: From<u16>>(v: u8) -> T {
    chaincast!(v; u16, T)
}

let x: u64 = widen(1);
```

Inside a fallible function, with `?` in the first argument:

```rust
fn load(input: Option<A>) -> Option<C> {
    let c: C = chaincast!(input?; B, C);
    Some(c)
}
```

The macro names no paths from `std`, so it also works in `#![no_std]` crates.

## Compared with `.into()`

The tempting shortcut is to chain `.into()` calls:

```rust
let c: C = a.into().into(); // error[E0282]: type annotations needed
```

That does **not** compile. `into()` is generic over its target, so inference has
no way to know what the middle step should be. You have to name the intermediate
type by hand:

```rust
let c: C = B::from(a).into();
// or
let c: C = Into::<B>::into(a).into();
```

`chaincast!` does exactly that naming for you, and keeps every step visible in
one place:

```rust
let c = chaincast!(a; B, C);
```

The result type is concrete, so no annotation is required. For a single
conversion, plain `.into()` is still shorter — `chaincast!` earns its keep at
two or more steps, or when the chain goes through generic or nested path types.

## Project layout

```text
chaincast/
├── Cargo.toml      # crate manifest + workspace root
├── README.md       # this file
├── src/lib.rs      # the chaincast! macro
├── example/        # small binary demonstrating the macro
└── LICENSE         # MIT
```

`example/` is a separate workspace member with `publish = false`, used as a
living usage sample rather than as library code.

## Development

```bash
cargo build                    # build the crate
cargo test --workspace         # run the test suite
cargo run -p example           # compile the usage example
cargo clippy --workspace --all-targets   # lints
cargo doc --open               # local docs
```

Requires Rust **1.85 or newer** (the crate uses the 2024 edition).

## Contributing

Issues and pull requests are welcome at
<https://github.com/r41ngee/chaincast>. Please keep changes small, and make sure
`cargo fmt --check`, `cargo clippy --workspace --all-targets` and
`cargo test --workspace` are clean before opening a PR.

## License

MIT — see [LICENSE](LICENSE). © 2026 ryoshin
