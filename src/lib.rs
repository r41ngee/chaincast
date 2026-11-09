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

#[macro_export]
macro_rules! chaincast_primitive {
    (
        $orig:expr;
        $($rest:ty),* $(,)?
    ) => {
        $orig $(as $rest)*
    };
}
