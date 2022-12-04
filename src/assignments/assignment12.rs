#![allow(single_use_lifetimes)]

//! Assignment 12: Concurrency.
//!
//! The primary goal of this assignment is to get used to concurrency.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-12.sh` works fine.
//! See `assignment12_grade.rs` and `/scripts/grade-12.sh` for the test script.

use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread;

use etrace::*;

/// The "pong" function (read the test script to figure out what it should do).
pub fn pong(rx1: &mut Receiver<u32>, tx2: &mut Sender<u32>) -> bool {
    todo!()
}

/// Executes the given functions (f1, f2) and returns the results.
pub fn use_scoped_thread<'scope, 'env, T1, T2, F1, F2>(
    s: &'scope thread::Scope<'scope, 'env>,
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
