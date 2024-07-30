//! Small exercises
//!
//! Refer `small_exercises_grade.rs` for test cases

use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;

use etrace::*;

/// The "pong" function
///
/// Data will be sent and received through `rx` and `tx`.
/// Read the `test_ping_pong` function in `small_exercises_grade.rs` to figure out what it should
/// do.
pub fn pong(rx1: &mut Receiver<u32>, tx2: &mut Sender<u32>) -> bool {
    todo!()
}

/// Executes the given functions (f1, f2) in concurrent and returns the results.
///
/// Read the `test_scoped_thread` function in `small_exercises_grade.rs` to figure out what it
/// should do.
pub fn use_scoped_thread<'scope, T1, T2, F1, F2>(
    s: &'scope thread::Scope<'scope, '_>,
    f1: F1,
    f2: F2,
) -> (T1, T2)
where
    T1: Send + 'scope,
    T2: Send + 'scope,
    F1: Send + FnOnce() -> T1 + 'scope,
    F2: Send + FnOnce() -> T2 + 'scope,
{
    todo!()
}
