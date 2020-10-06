mod algebra {
    pub use num_traits::Zero;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Matrix<T> {
        pub n: usize,      // line           [* * * * *]
        pub m: usize,      // column         [* * * * *] -> n = 3, m = 5
        pub array: Vec<T>, //                [* * * * *]
    }

    impl<T> Matrix<T>
    where
        T: Zero + Clone,
    {
        pub fn new(n: usize, m: usize) -> Self {
            Matrix {
                n,
                m,
                array: vec![T::zero(); n * m],
            }
        }

        pub fn append(n: usize, m: usize, array: Vec<T>) -> Self {
            if array.len() != n * m {
                panic!("`Matrix::append` needs appropriately sized Vec<T>");
            }
            Matrix { n, m, array }
        }

        pub fn append_line(vec: Vec<Vec<T>>) -> Self {
            let n = vec.len();
            let m = vec[0].len();
            if !vec.iter().all(|e| e.len() == m) {
                panic!("`Matrix::append_line` needs appropriatly sized Vec<Vec<T>>");
            }
            Matrix {
                n,
                m,
                array: vec.concat(),
            }
        }

        pub fn append_column(vec: Vec<Vec<T>>) -> Self {
            let n = vec[0].len();
            let m = vec.len();
            if !vec.iter().all(|e| e.len() == n) {
                panic!("`Matrix::append_column` needs appropriatly sized Vec<Vec<T>>>");
            }
            Matrix {
                n,
                m,
                array: {
                    let mut v = Vec::new();
                    for j in 0..n {
                        for i in 0..m {
                            v.push((vec[i][j]).clone());
                        }
                    }
                    v
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algebra::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Matrix::<f32>::new(3, 4),
            Matrix {
                n: 3,
                m: 4,
                array: vec![0.0; 12]
            }
        ); // TODO: Implement appropriate test
    }

    #[test]
    fn test_append() {
        assert_eq!(
            Matrix::append(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
            Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::append` needs appropriately sized Vec<T>")]
    fn test_append_panic() {
        let _dummy_matrix = Matrix::append(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_append_line() {
        assert_eq!(
            Matrix::append_line(vec![
                vec![1, 2, 3],
                vec![4, 5, 6],
                vec![7, 8, 9],
                vec![10, 11, 12]
            ]),
            Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::append_line` needs appropriatly sized Vec<Vec<T>>")]
    fn test_append_line_panic() {
        let _dummy_matrix =
            Matrix::append_line(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8], vec![9]]);
    }

    #[test]
    fn test_append_column() {
        assert_eq!(
            Matrix::append_column(vec![
                vec![1, 4, 7, 10],
                vec![2, 5, 8, 11],
                vec![3, 6, 9, 12]
            ]),
            Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::append_column` needs appropriatly sized Vec<Vec<T>>")]
    fn test_append_column_panic() {
        let _dummy_matrix =
            Matrix::append_column(vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9]]);
    }
}
