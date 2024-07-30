//! You will implement a number of custom operators.

/// Custom option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyOption<T> {
    /// Some value of type `T`.
    MySome(T),
    /// No value.
    MyNone,
}

/// Maps an `MyOption<T>` to `MyOption<U>` by applying a function to a contained value.
///
/// # Examples
///
/// Converts an `MyOption<String>` into an `MyOption<usize>`, consuming the original:
///
/// ```
/// use cs220::assignments::assignment03::custom_operators::*;
///
/// fn len(s: String) -> usize {
///     s.len()
/// }
///
/// assert_eq!(my_map(MyOption::MySome(String::from("Hello, World!")), len), MyOption::MySome(13));
/// assert_eq!(my_map(MyOption::MyNone, len), MyOption::MyNone);
/// ```
pub fn my_map<T, U, F: FnOnce(T) -> U>(v: MyOption<T>, f: F) -> MyOption<U> {
    todo!()
}

/// Returns `MyNone` if the option is `MyNone`, otherwise calls `f` with the wrapped value and
/// returns the result.
///
/// Some languages call this operation flatmap.
///
/// # Examples
///
/// ```
/// use cs220::assignments::assignment03::custom_operators::*;
///
/// fn pos_then_to_string(x: isize) -> MyOption<String> {
///     if x > 0 {
///         MyOption::MySome(x.to_string())
///     } else {
///         MyOption::MyNone
///     }
/// }
///
/// assert_eq!(my_and_then(MyOption::MySome(2), pos_then_to_string), MyOption::MySome(2.to_string()));
/// assert_eq!(my_and_then(MyOption::MySome(-3), pos_then_to_string), MyOption::MyNone);
/// assert_eq!(my_and_then(MyOption::MyNone, pos_then_to_string), MyOption::MyNone);
/// ```
pub fn my_and_then<T, U, F: FnOnce(T) -> MyOption<U>>(v: MyOption<T>, f: F) -> MyOption<U> {
    todo!()
}

/// Custom operator: `option_op_or(v1, v2, f)`. If neither `v1` nor `v2` is `Some`, returns `None`.
/// If exactly one is `Some`, returns the same `Some` value. If both are `Some`, apply the values
/// inside `Some` to `f` and wrap the resulting value inside `Some`.
///
/// # Examples
///
/// ```
/// use cs220::assignments::assignment03::custom_operators::*;
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
///
/// assert_eq!(my_option_op_or(MyOption::MyNone, MyOption::MyNone, product), MyOption::MyNone);
/// assert_eq!(my_option_op_or(MyOption::MySome(3), MyOption::MyNone, product), MyOption::MySome(3));
/// assert_eq!(my_option_op_or(MyOption::MySome(3), MyOption::MySome(5), product), MyOption::MySome(15));
/// ```
pub fn my_option_op_or<T, F: FnOnce(T, T) -> T>(
    v1: MyOption<T>,
    v2: MyOption<T>,
    f: F,
) -> MyOption<T> {
    todo!()
}
