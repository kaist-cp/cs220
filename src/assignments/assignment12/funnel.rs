//! Funnel
//!
//! Spawn a thread that executes a funnel.
//!
//! Funnel will receive data from multiple receivers and send it to a single sender. Also, the
//! funnel will filter out data that does not pass the filter function.
//!
//! Refer to `funnel_grade.rs` for test cases.

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

/// Spawn a thread that concurrently receive datas from `rxs`, send it to `tx` if it makes `f` true.
/// Returns its handle.
pub fn spawn_funnel<T, F>(rxs: Vec<Receiver<T>>, tx: Sender<T>, f: F) -> JoinHandle<()>
where
    T: Send + 'static,
    F: Send + Sync + Fn(&T) -> bool + 'static,
{
    todo!()
}
