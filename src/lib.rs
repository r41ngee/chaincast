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
