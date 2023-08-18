//! Simple Turing machien emulator
//!
//! Simple One-head, One-tape turing machine
//! See <https://en.wikipedia.org/wiki/Turing_machine> that describes what turing machine is.
//! See `test_turing_machine` module in `assignment11_grade.rs` for examples.
//!
//! Goal: To be accustomed with `RefCell`, `HashMap`
//!
//! Refer `turing_machine.rs` for test cases.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Formatter};
use thiserror::Error;

/// Error type for Turing machine
/// <https://google.github.io/comprehensive-rust/error-handling/deriving-error-enums.html>
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TuringMachineError {
    /// Invalid movement
    /// You can't move left from the leftmost location
    /// or move right from the rightmost location.
    #[error("Invalid movement")]
    InvalidMovement,

    /// Exceeded maximum steps
    #[error("Exceeded maximum steps")]
    ExceedMaxSteps,

    /// Invalid state or value
    /// Occurs when you cannot find instruction for the current state and value
    #[error("Invalid state or value")]
    InvalidStateOrValue,
}

/// Turing Machine implementation
#[derive(Debug)]
pub struct TuringMachine<TMState, TMValue>
where
    TMState: Default + Eq + PartialEq + std::hash::Hash + Clone,
    TMValue: Eq + PartialEq + std::hash::Hash + Clone,
{
    /// Number of steps taken by the Turing machine
    pub steps: RefCell<usize>,

    /// Table of instructions for the Turing machine
    pub table: HashMap<(TMState, TMValue), (TMState, Move, TMValue)>,

    /// Tape of the Turing machine. Finite length
    pub tape: Vec<RefCell<TMValue>>,
}

/// Implementation of the movement instructions of the head of the tape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Move {
    /// Move Left
    L,
    /// Move Right
    R,
    /// Don't move
    N,
}

/// Cursor for Turing machine
#[derive(Debug)]
pub struct Cursor<'a, TMState, TMValue>
where
    TMState: Default + Eq + PartialEq + std::hash::Hash + Clone,
    TMValue: Eq + PartialEq + std::hash::Hash + Clone,
{
    // Turing mahcine
    tm: &'a TuringMachine<TMState, TMValue>,
    // Index of the tape
    index: usize,
    // Current state of the Turing machine
    state: TMState,
}

impl<'a, TMState, TMValue> Cursor<'a, TMState, TMValue>
where
    TMState: Default + Eq + PartialEq + std::hash::Hash + Clone,
    TMValue: Eq + PartialEq + std::hash::Hash + Clone,
{
    /// Generate new cursor
    pub fn new(tm: &'a TuringMachine<TMState, TMValue>, state: TMState, index: usize) -> Self {
        Cursor { tm, index, state }
    }

    /// Run the Turing machine until it halts (if it halts). Print every step of that.
    pub fn run(&mut self, max_step: usize) -> Result<(TMValue, usize), TuringMachineError> {
        let mut steps = self.tm.steps.borrow_mut();
        while self.state != TMState::default() {
            *steps += 1;
            if *steps > max_step {
                return Err(TuringMachineError::ExceedMaxSteps);
            }
            self.step()?;
            // println!("{}", self);
        }
        Ok((self.get(), *steps))
    }

    /// Set tape value at the current index with `value`
    /// You may need this function for `mov` function
    fn set(&mut self, value: TMValue) -> TMValue {
        todo!();
    }

    /// Step the Turing machine
    /// Look at the `run` function to see how this function is used.
    fn step(&mut self) -> Result<(), TuringMachineError> {
        todo!();
    }

    /// Move the cursor while setting the value of the current index
    fn mov(&mut self, new_value: TMValue, movement: &Move) -> Result<(), TuringMachineError> {
        todo!();
    }

    /// Get the value of the current index
    /// Look at the `run` function to see how this function is used.
    fn get(&self) -> TMValue {
        todo!();
    }
}

impl<TMState, TMValue> TuringMachine<TMState, TMValue>
where
    TMState: Default + Eq + PartialEq + std::hash::Hash + Clone,
    TMValue: Eq + PartialEq + std::hash::Hash + Clone,
{
    /// Generate new Turing machine
    pub fn new(
        table: HashMap<(TMState, TMValue), (TMState, Move, TMValue)>,
        tape: Vec<RefCell<TMValue>>,
    ) -> Self {
        todo!()
    }
}
