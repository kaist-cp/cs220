//! Syntax.

/// Command of the form "<expression>" or "<var> = <expression>".
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// Variable (lhs).
    pub variable: Option<String>,
    /// Expression (rhs).
    pub expression: Expression,
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    /// Add.
    Add,
    /// Subtract.
    Subtract,
    /// Multiply.
    Multiply,
    /// Divide.
    Divide,
    /// Power.
    Power,
}

/// Expression.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Number.
    Num(f64),
    /// Variable.
    Variable(String),
    /// Binary operation.
    BinOp {
        /// Operator.
        op: BinOp,
        /// Lhs.
        lhs: Box<Expression>,
        /// Rhs.
        rhs: Box<Expression>,
    },
}
