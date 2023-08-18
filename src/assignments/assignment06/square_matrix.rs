//! Square matrix

/// Square matrix
pub trait SquareMatrix {
    /// The type of the submatrix of this square matrix.
    /// For example, the submatrix of a 3 x 3 matrix is a 2 x 2 matrix.
    /// https://en.wikipedia.org/wiki/Matrix_(mathematics)#Submatrix
    type Submatrix;

    /// Returns the submatrix obtained by removing the `row`th row and `col`th column
    /// from the original matrix.
    /// https://en.wikipedia.org/wiki/Matrix_(mathematics)#Submatrix
    fn sub_matrix(&self, row: usize, col: usize) -> Self::Submatrix;

    /// Returns the determinant of the matrix.
    fn det(&self) -> i64;

    /// Returns the determinant of ab, where a is self, b is given, and ab is the matrix product of them.
    /// Note that the size of a and b are the same.
    /// Hint: Use the fact that det(ab) = det(a) * det(b)
    /// https://en.wikipedia.org/wiki/Determinant#Multiplicativity_and_matrix_groups
    fn det_ab(&self, b: &Self) -> i64 {
        todo!()
    }
}

/// 2 x 2 matrix
#[derive(Debug, PartialEq)]
pub struct Mat2 {
    /// inner is a 2 dimensional array (size: 2 x 2)
    pub inner: [[i64; 2]; 2],
}

impl SquareMatrix for Mat2 {
    type Submatrix = i64;

    fn sub_matrix(&self, row: usize, col: usize) -> Self::Submatrix {
        // Hint: The submatrix of a 2 x 2 matrix is simply a single number.
        todo!()
    }

    // Hint: https://en.wikipedia.org/wiki/Determinant
    fn det(&self) -> i64 {
        todo!()
    }
}

/// 3 x 3 matrix
#[derive(Debug, PartialEq)]
pub struct Mat3 {
    /// inner is a 2 dimensional array (size: 3 x 3)
    pub inner: [[i64; 3]; 3],
}

impl SquareMatrix for Mat3 {
    type Submatrix = Mat2;

    fn sub_matrix(&self, row: usize, col: usize) -> Self::Submatrix {
        todo!()
    }

    // Hint: Use the determinant of the sub-matrices.
    // https://semath.info/src/determinant-three-by-three.html
    fn det(&self) -> i64 {
        todo!()
    }
}

/// 4 x 4 matrix
#[derive(Debug, PartialEq)]
pub struct Mat4 {
    /// inner is a 2 dimensional array (size: 4 x 4)
    pub inner: [[i64; 4]; 4],
}

impl SquareMatrix for Mat4 {
    type Submatrix = Mat3;

    fn sub_matrix(&self, row: usize, col: usize) -> Self::Submatrix {
        todo!()
    }

    // Hint: Use the determinant of the sub-matrices.
    // https://semath.info/src/determinant-four-by-four.html
    fn det(&self) -> i64 {
        todo!()
    }
}
