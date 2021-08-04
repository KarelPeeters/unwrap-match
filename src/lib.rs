/// Like `Option::unwrap` but for arbitrary patterns.
/// ```
/// # use unwrap_match::unwrap_match;
/// assert_eq!(5, unwrap_match!(Some(5), Some(x) => x));
/// ```
/// ```should_panic
/// # use unwrap_match::unwrap_match;
/// // this will panic
/// unwrap_match!(None, Some(x) => x);
/// ```
#[macro_export]
macro_rules! unwrap_match {
    ($value: expr, $($pattern: pat)|+ => $result: expr) => {
        match $value {
            $($pattern)|+ =>
                $result,
            ref value =>
                panic!("unwrap_match failed: `{:?}` does not match `{}`", value, stringify!($($pattern)|+)),
        }
    };
}