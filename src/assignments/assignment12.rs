#![allow(single_use_lifetimes)]

//! Assignment 12: Concurrency.
//!
//! The primary goal of this assignment is to get used to concurrency.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-12.sh` works fine.
//! See `assignment12_grade.rs` and `/scripts/grade-12.sh` for the test script.

use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use etrace::*;

/// The "pong" function (read the test script to figure out what it should do).
pub fn pong(rx1: &mut Receiver<u32>, tx2: &mut Sender<u32>) -> bool {
    todo!()
}

/// Executes the given functions (f1, f2) in concurrent and returns the results.
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

/// Spawns a thread that concurrently receive datas from `rxs`, send it to `tx` if it makes `f` true. Returns its handle.
pub fn spawn_funnel<T, F>(rxs: Vec<Receiver<T>>, tx: Sender<T>, f: F) -> JoinHandle<()>
where
    T: Send + 'static,
    F: Send + Sync + Fn(&T) -> bool + 'static,
{
    todo!()
}

/// Sender for `demux`.
#[derive(Debug)]
pub struct DemuxSender<T, F: Fn(&T) -> bool> {
    tx_true: Sender<T>,
    tx_false: Sender<T>,
    f: F,
}

impl<T, F: Fn(&T) -> bool> DemuxSender<T, F> {
    /// If `f(&value)` is true, send `value` to `tx_true`. Otherwise, send `value` to `tx_false`.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        todo!()
    }
}

/// Demux.
///
/// It returns one sender and two receivers. For all data sent on the `DemuxSender`, if it makes `f` true, it will become
/// available on the first `Receiver`. Otherwise, it will become available on the second `Receiver`.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment12::*;
///
/// use std::thread;
///
/// fn main() {
///     let (tx, rx1, rx2) = demux::<usize, _>(|x| x % 2 == 0);
///
///     thread::spawn(move || {
///         tx.send(1).unwrap();
///         tx.send(2).unwrap();
///     });
///
///     assert_eq!(rx1.recv().unwrap(), 2);
///     assert_eq!(rx2.recv().unwrap(), 1);
/// }
/// ```
pub fn demux<T, F: Fn(&T) -> bool>(f: F) -> (DemuxSender<T, F>, Receiver<T>, Receiver<T>) {
    todo!()
}
