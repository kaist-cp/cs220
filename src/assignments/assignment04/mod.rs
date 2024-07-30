//! Assignment 4: Designing a calculator.
//!
//! The primary goal of this assignment is twofold:
//! - (1) understanding the `pest` third-party library from documentations; and
//! - (2) using programming concepts you've learned so far to implement a simple arithmetic
//!   calculator.
//!
//! For `pest`, read the following documentations (that contain 90% of the solution):
//! - <https://pest.rs/>
//! - <https://pest.rs/book/>
//! - <https://docs.rs/pest/latest/pest>
//!
//! For calculator, just reading `syntax.rs` would suffice for you to understand what to do.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade.sh 4` works
//! fine. See `assignment04/grade.rs` and `/scripts/grade.sh 4` for the test script.
//!
//! Run `/scripts/prepare-submissions.sh` and submit `/target/assignment04.zip` to <https://gg.kaist.ac.kr>.
//!
//! To submit, run
//! ```bash
//! # At the cs220 home directory,
//! ./scripts/submit.sh
//! ```
//! and submit the generated `assignment04.zip` file in `target` directory.

pub mod context;
mod grade;
pub mod parser;
pub mod syntax;
