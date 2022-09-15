//! Parser.

use super::syntax::*;
use anyhow::Result;

#[allow(missing_docs)]
#[allow(missing_debug_implementations)]
mod inner {
    use pest_derive::*;

    #[derive(Parser)]
    #[grammar = "assignments/assignment04/syntax.pest"]
    pub(crate) struct SyntaxParser;
}

/// Parses command.
pub fn parse_command(line: &str) -> Result<Command> {
    todo!("fill here")
}
