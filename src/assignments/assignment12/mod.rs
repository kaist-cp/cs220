//! Assignment 12: Concurrency.
//!
//! The primary goal of this assignment is to get used to concurrency.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade.sh 12` works
//! fine. See `assignment12/*_grade.rs` and `/scripts/grade.sh 12` for the test script.
//!
//! To submit, run
//! ```bash
//! # At the cs220 home directory,
//! ./scripts/submit.sh
//! ```
//! and submit the generated `assignment12.zip` file in `target` directory.

pub mod card;
pub mod demux;
pub mod funnel;
pub mod small_exercises;

mod card_grade;
mod demux_grade;
mod funnel_grade;
mod small_exercises_grade;
