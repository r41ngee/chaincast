#![doc = include_str!("../README.md")]

/// Converts a value through a chain of [`From`] implementations.
///
/// `chaincast!(x; B, C)` expands to `C::from(B::from(x))`: one
/// `<Type>::from(…)` call per type in the list, applied left to right.
/// That keeps a multistep conversion readable, without nesting calls
/// inside-out or spelling out the intermediate type for every `.into()`.
///
/// # Syntax
///
/// ```text
/// chaincast!(expression; Type1, Type2, ..., TypeN)
/// ```
///
/// The `;` separates the value from the list of types to convert it through.
/// The macro evaluates to the value of the last type in the list; a trailing
/// comma is allowed.
///
/// # Requirements
///
/// Every step needs a `From` impl: `Type1: From<T>`, where `T` is the type of
/// `expression`, and `TypeN: From<TypeN-1>` for each subsequent step. Only
/// infallible conversions are covered — use `TryFrom` for fallible ones.
///
/// # Examples
///
/// ```
/// use chaincast::chaincast;
///
/// struct A;
/// struct B;
/// struct C;
///
/// impl From<A> for B {
///     fn from(_: A) -> B {
///         B
///     }
/// }
///
/// impl From<B> for C {
///     fn from(_: B) -> C {
///         C
///     }
/// }
///
/// // The result type is concrete, so no annotation is needed.
/// let _c = chaincast!(A; B, C);
/// ```
///
/// Chains through standard library types work the same way:
///
/// ```
/// use std::ffi::OsString;
/// use std::path::PathBuf;
///
/// use chaincast::chaincast;
///
/// let path = chaincast!(String::from("/tmp"); OsString, PathBuf);
/// assert_eq!(path, PathBuf::from("/tmp"));
/// ```
///
/// The macro names no paths of its own, so it also works in `#![no_std]`
/// crates, and its internal binding is hygienic.
///
/// See [`chaincast_primitive!`] for casting primitives with `as`.
#[macro_export]
macro_rules! chaincast {
    (
        $orig:expr;
        $($rest:ty),* $(,)?
    ) => {{
        let __inner_result = $orig;
        $(
        let __inner_result = <$rest>::from(__inner_result);
        )*
        __inner_result
    }};
}

/// Chains `as` casts: `chaincast_primitive!(x; u8, i8)` expands to
/// `x as u8 as i8`.
///
/// Unlike `chaincast!`, this macro needs no `From` impls — it is purely
/// syntactic and works with any types the `as` operator accepts (numeric
/// primitives, `char`, `bool`, pointers, and so on). Numeric casts follow
/// `as` semantics: integer-to-integer casts truncate to the low bits of the
/// target width and never fail, while float-to-int casts saturate. See
/// `example/src/primitive.rs` for a worked example where casting
/// `-2000i32` through `u8` yields `48`.
#[macro_export]
macro_rules! chaincast_primitive {
    (
        $orig:expr;
        $($rest:ty),* $(,)?
    ) => {
        $orig $(as $rest)*
    };
}

#[macro_export]
macro_rules! try_chaincast {
    (
        $orig:expr;
        $($rest:ty),* $(,)?
    ) => {{
        let __closed: Result<_, Box<dyn Error>> || {
            let __inner_result: $orig;
            $(
            let __inner_result = <$rest>::try_from(__inner_result)?;
            )*
            __inner_result
        };
        __closed
    }};
}
