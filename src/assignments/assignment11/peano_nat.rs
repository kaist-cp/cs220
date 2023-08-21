//! Peano natural number.

/// We can represent any natural number using only two symbols: 0 and S.
///
/// E.g.
///     O == 0
///     S(O) == 1
///     S(S(O)) == 2
///     ... so on.
#[derive(Debug, Clone, PartialEq)]
pub enum Nat {
    /// Zero
    O,
    /// Plus one
    S(Box<Nat>),
}

impl Nat {
    /// Create `Nat` from `usize`
    pub fn from_usize(n: usize) -> Nat {
        todo!()
    }

    /// Convert `Nat` into nonnegative integer
    pub fn as_usize(&self) -> usize {
        todo!()
    }
}

// Implement `Add` operator (i.e. `+`) for `Nat`.
impl std::ops::Add for Nat {
    type Output = Nat;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// Implement `Sub` operator (i.e. `-`) for `Nat`.
// If the result is negative, return `Nat::O`.
impl std::ops::Sub for Nat {
    type Output = Nat;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// Implement `Mul` operator (i.e. `*`) for `Nat`.
impl std::ops::Mul for Nat {
    type Output = Nat;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
