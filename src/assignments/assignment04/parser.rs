#![allow(deprecated)]

//! Parser.

use anyhow::{bail, Result};
use etrace::*;
use lazy_static::*;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::*;
use pest::Parser;

use super::syntax::*;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

use inner::*;

/// Parses command.
///
/// ## Operator Associativty
///
/// For associativity of each operator, please follow [here](https://docs.rs/pest/latest/pest/prec_climber/struct.PrecClimber.html#examples).
///
/// e.g. `1+2+3` should be parsed into `(1+2)+3`, not `1+(2+3)` because the associativity of
/// plus("add" in our hw) operator is `Left`.
pub fn parse_command(line: &str) -> Result<Command> {
    todo!("fill here")
}
