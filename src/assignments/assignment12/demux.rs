//! Demultiplexing sender
//!
//! Implement a demultiplexing sender.
//!
//! Demultiplexer, `Demux` in short, is a device that has one input and many outputs.
//! It distributes the input to the outputs according to the control signal.
//! It is used when a circuit wishes to send a signal to one of many devices.
//! For more information, refer <https://www.electronics-tutorials.ws/combination/comb_3.html>
//!
//! In this assignment, closure `f` will be given as an argument to `demux` function.
//! This closure will be used to determine which destination to send the input data.
//!
//! Refer `demux_grade.rs` for test cases

use std::sync::mpsc::{channel, Receiver, SendError, Sender};
use std::thread;

/// Sender for demux.
#[derive(Debug)]
pub struct DemuxSender<T, F: Fn(&T) -> bool> {
    tx_true: Sender<T>,
    tx_false: Sender<T>,
    f: F,
}

impl<T, F: Fn(&T) -> bool> DemuxSender<T, F> {
    /// send
    ///
    ///  If `f(&value)` is true, send `value` to `tx_true`. Otherwise, send `value` to `tx_false`.
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        todo!()
    }
}

/// Demux.
pub fn demux<T, F: Fn(&T) -> bool>(f: F) -> (DemuxSender<T, F>, Receiver<T>, Receiver<T>) {
    todo!()
}
