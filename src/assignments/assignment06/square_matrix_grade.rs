#[cfg(test)]
mod test {
    use super::super::square_matrix::*;
    use ntest::assert_about_eq;

    #[test]
    fn test_mat2() {
        let mat = Mat2 {
            inner: [[1, 2], [3, 4]],
        };
        assert_eq!(mat.sub_matrix(1, 1), 4);
        assert_eq!(mat.sub_matrix(1, 2), 3);
        assert_eq!(mat.sub_matrix(2, 1), 2);
        assert_eq!(mat.sub_matrix(2, 2), 1);
        assert_eq!(mat.det(), -2);

        let mat2 = Mat2 {
            inner: [[2, 3], [5, 7]],
        };
        assert_eq!(mat.det_ab(&mat2), mat.det() * mat2.det());
    }

    #[test]
    fn test_mat3() {
        let mat = Mat3 {
            inner: [[1, 2, 3], [5, 5, 6], [7, 8, 10]],
        };
        assert_eq!(
            mat.sub_matrix(1, 2),
            Mat2 {
                inner: [[5, 6], [7, 10]]
            }
        );
        assert_eq!(mat.det(), 1);

        let mat2 = Mat3 {
            inner: [[2, 3, 5], [7, 10, 11], [12, 14, 20]],
        };
        assert_eq!(mat2.det(), -42);
        assert_eq!(mat.det_ab(&mat2), mat.det() * mat2.det());
    }

    #[test]
    fn test_mat4() {
        let mat = Mat4 {
            inner: [
                [1, 11, 3, 4],
                [5, 6, 7, 9],
                [25, 10, 11, 20],
                [36, 14, 15, 30],
            ],
        };
        assert_eq!(
            mat.sub_matrix(2, 3),
            Mat3 {
                inner: [[1, 11, 4], [25, 10, 20], [36, 14, 30]]
            }
        );
        assert_eq!(mat.det(), 2089);

        let mat2 = Mat4 {
            inner: [
                [2, 3, 5, 5],
                [7, 10, 11, 20],
                [12, 14, 20, 30],
                [1, 2, 5, 10],
            ],
        };
        assert_eq!(mat2.det(), -340);
        assert_eq!(mat.det_ab(&mat2), mat.det() * mat2.det());
    }
}
