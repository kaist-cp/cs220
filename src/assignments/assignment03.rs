//! Assignment 3: Mastering common programming concepts (2/2).
//!
//! The primary goal of this assignment is to re-learn the common programming concepts in Rust, especially those in the Rust Book chapters 6, 7, 8, and 9.
//! Please make sure you're comfortable with the concepts to proceed on to the next assignments.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-03.sh` works fine.
//! See `assignment03_grade.rs` and `/scripts/grade-03.sh` for the test script.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    todo!()
}

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
/// use cs220::assignments::assignment03::{my_map, MyOption};
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

/// Returns `MyNone` if the option is `MyNone`, otherwise calls `f` with the wrapped value and returns the result.
///
/// Some languages call this operation flatmap.
///
/// # Examples
///
/// ```
/// use cs220::assignments::assignment03::{MyOption, my_and_then};
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

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```ignore
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```ignore
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    todo!()
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    todo!()
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    todo!()
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```ignore
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```ignore
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add {person} to {department}", "Remove {person} from {department}", and "Move {person} from {department} to {department}".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    todo!()
}

/// Parse the string as a shell command.
///
/// Usually, a shell command is whitespace-separated array of strings.
/// ```text
/// cat file  -->  ["cat", "file"]
/// ```
/// But sometimes, you may want to include whitespaces in each argument.
/// In that case, you can use quotes.
/// ```text
/// ls 'VirtualBox VMs'  -->  ["ls", 'VirtualBox VMs']
/// ls VirtualBox' 'VMs  -->  ["ls", 'VirtualBox VMs']
/// ```
///
/// For simplicity, you may assume that the string only contains alphanumeric characters, spaces
/// (" "), and single quotes ("'").
pub fn parse_shell_command(command: &str) -> Vec<String> {
    todo!()
}

/// Represents a JSON value. See https://en.wikipedia.org/wiki/JSON.
///
/// For simplicity, you may assume that numbers are of type `i64`, and strings do not contain
/// special characters that need to be escaped (e.g. '"', '\n', ...).
#[derive(Debug, PartialEq, Eq)]
pub enum JsonValue {
    /// null
    Null,
    /// true, false
    Boolean(bool),
    /// integers
    Number(i64),
    /// strings
    String(String),
    /// array of JSON values
    Array(Vec<JsonValue>),
    /// objects
    Object(HashMap<String, JsonValue>),
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => write!(f, "null"),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Array(arr) => {
                write!(f, "[")?;
                let mut iter = arr.iter();
                if let Some(item) = iter.next() {
                    write!(f, "{}", item)?;
                    for item in iter {
                        write!(f, ", {}", item)?;
                    }
                }
                write!(f, "]")
            }
            JsonValue::Object(obj) => {
                write!(f, "{{")?;
                let mut iter = obj.iter();
                if let Some((key, value)) = iter.next() {
                    write!(f, "\"{}\": {}", key, value)?;
                    for (key, value) in iter {
                        write!(f, ", \"{}\": {}", key, value)?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

/// Parse a string into a JSON value. Returns `Err(())` if it contains syntax errors.
pub fn parse_json(json_string: &str) -> Result<JsonValue, ()> {
    todo!()
}
