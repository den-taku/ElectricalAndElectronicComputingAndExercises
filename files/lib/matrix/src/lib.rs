mod algebra {
    pub use num_traits::Zero;
    pub use std::ops::{Add, BitAnd, Neg, Not, Sub};

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
                panic!("`Matrix::append` needs appropriately sized Vec<T>.");
            }
            Matrix { n, m, array }
        }

        pub fn append_line(vec: Vec<Vec<T>>) -> Self {
            let n = vec.len();
            let m = vec[0].len();
            if !vec.iter().all(|e| e.len() == m) {
                panic!("`Matrix::append_line` needs appropriatly sized Vec<Vec<T>>.");
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
                panic!("`Matrix::append_column` needs appropriatly sized Vec<Vec<T>>.");
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

    impl<T> Neg for Matrix<T>
    where
        T: Neg<Output = T> + Clone,
    {
        type Output = Self;
        fn neg(self) -> Self {
            let new_field = self.array.iter().map(|e| e.clone().neg()).collect();
            Matrix {
                array: new_field,
                ..self
            }
        }
    }

    impl<T> Not for Matrix<T>
    where
        T: Not<Output = T> + Clone,
    {
        type Output = Self;
        fn not(self) -> Self {
            let new_field = self.array.iter().map(|e| e.clone().not()).collect();
            Matrix {
                array: new_field,
                ..self
            }
        }
    }

    impl<T> Add for &Matrix<T>
    where
        T: Add<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn add(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::add` needs two Matrix<T> the same sized.");
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() + rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> Sub for &Matrix<T>
    where
        T: Add<Output = T> + Neg<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::sub` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() + (-rhs.array[i].clone()))
                    }
                    v
                },
            }
        }
    }

    impl<T> BitAnd for &Matrix<T>
    where
        T: BitAnd<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn bitand(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitand` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() & rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }
}

// TEST
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
    #[should_panic(expected = "`Matrix::append` needs appropriately sized Vec<T>.")]
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
    #[should_panic(expected = "`Matrix::append_line` needs appropriatly sized Vec<Vec<T>>.")]
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
    #[should_panic(expected = "`Matrix::append_column` needs appropriatly sized Vec<Vec<T>>.")]
    fn test_append_column_panic() {
        let _dummy_matrix =
            Matrix::append_column(vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9]]);
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            -Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![-1, -2, -3, -4, -5, -6, -7, -8, -9, -10, -11, -12]
            }
        );
        assert_eq!(
            Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .neg(),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.neg(),
                    2.neg(),
                    3.neg(),
                    4.neg(),
                    5.neg(),
                    6.neg(),
                    7.neg(),
                    8.neg(),
                    9.neg(),
                    10.neg(),
                    11.neg(),
                    12.neg()
                ]
            }
        );
    }
    #[test]
    fn test_not() {
        assert_eq!(
            !Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false
                ]
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    !true, !true, !false, !false, !true, !false, !true, !false, !false, !true,
                    !true, !false
                ]
            }
        );
        assert_eq!(
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false
                ]
            }
            .not(),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true.not(),
                    true.not(),
                    false.not(),
                    false.not(),
                    true.not(),
                    false.not(),
                    true.not(),
                    false.not(),
                    false.not(),
                    true.not(),
                    true.not(),
                    false.not()
                ]
            }
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } + &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 + 1,
                    2 + 2,
                    3 + 3,
                    4 + 4,
                    5 + 5,
                    6 + 6,
                    7 + 7,
                    8 + 8,
                    9 + 9,
                    10 + 10,
                    11 + 11,
                    12 + 12
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .add(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            })),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.add(1),
                    2.add(2),
                    3.add(3),
                    4.add(4),
                    5.add(5),
                    6.add(6),
                    7.add(7),
                    8.add(8),
                    9.add(9),
                    10.add(10),
                    11.add(11),
                    12.add(12)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::add` needs two Matrix<T> the same sized.")]
    fn test_add_panic() {
        let _dummy_matrix = &Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        } + &Matrix {
            n: 4,
            m: 3,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } - &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 - 1,
                    2 - 2,
                    3 - 3,
                    4 - 4,
                    5 - 5,
                    6 - 6,
                    7 - 7,
                    8 - 8,
                    9 - 9,
                    10 - 10,
                    11 - 11,
                    12 - 12
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .sub(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            })),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.sub(1),
                    2.sub(2),
                    3.sub(3),
                    4.sub(4),
                    5.sub(5),
                    6.sub(6),
                    7.sub(7),
                    8.sub(8),
                    9.sub(9),
                    10.sub(10),
                    11.sub(11),
                    12.sub(12)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::sub` needs two Matrix<T> the same sized.")]
    fn test_sub_panic() {
        let _dummy_matrix = &Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        } - &Matrix {
            n: 4,
            m: 3,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_bitand() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false
                ]
            } & &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true
                ]
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true & true,
                    true & false,
                    false & false,
                    false & true,
                    true & true,
                    false & false,
                    true & false,
                    false & true,
                    false & false,
                    true & true,
                    true & false,
                    false & true
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false
                ]
            }
            .bitand(&Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, false, false, true, true, false, false, true, false, true, false, true
                ]
            })),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true.bitand(true),
                    true.bitand(false),
                    false.bitand(false),
                    false.bitand(true),
                    true.bitand(true),
                    false.bitand(false),
                    true.bitand(false),
                    false.bitand(true),
                    false.bitand(false),
                    true.bitand(true),
                    true.bitand(false),
                    false.bitand(true)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::bitand` needs two Matrix<T> the same sized.")]
    fn test_bitand_panic() {
        let _dummy_matrix = &Matrix {
            n: 3,
            m: 4,
            array: vec![
                true, true, false, false, true, false, true, false, false, true, true, false,
            ],
        } & &Matrix {
            n: 4,
            m: 3,
            array: vec![
                true, false, false, true, true, false, false, true, false, true, false, true,
            ],
        };
    }
}
