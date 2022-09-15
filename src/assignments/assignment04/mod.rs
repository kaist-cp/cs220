//! Assignment 4: Designing a calculator.
//!
//! The primary goal of this assignment is twofold:
//! (1) understanding the `pest` third-party library from documentations; and
//! (2) using programming concepts you've learned so far to implement a simple arithmetic calculator.
//!
//! For `pest`, read the following documentations (that contain 90% of the solution):
//! - <https://pest.rs/>
//! - <https://pest.rs/book/>
//! - <https://docs.rs/pest/latest/pest>
//!
//! For calculator, just reading `syntax.rs` would suffice for you to understand what to do.
//!
//! You should fill out the `todo!()` placeholders in such a way that `/scripts/grade-04.sh` works fine.
//! See `assignment04_grade.rs` and `/scripts/grade-04.sh` for the test script.
//! Run `/scripts/prepare-submissions.sh` and submit `/target/assignment04.zip` to <https://gg.kaist.ac.kr>.

pub mod context;
pub mod parser;
pub mod syntax;
