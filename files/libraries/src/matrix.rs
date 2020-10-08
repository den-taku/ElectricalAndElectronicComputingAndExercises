mod algebra {
    pub use num_traits::Zero;
    pub use std::ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index,
        IndexMut, Neg, Not, Shl, Shr, Sub, SubAssign,
    };

    #[derive(Clone, Debug, PartialEq, PartialOrd)]
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

    impl<T> BitOr for &Matrix<T>
    where
        T: BitOr<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn bitor(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitor` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() | rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> BitXor for &Matrix<T>
    where
        T: BitXor<Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn bitxor(self, rhs: Self) -> Self::Output {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitxor` needs two Matrix<T> the same sized.")
            }
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() ^ rhs.array[i].clone())
                    }
                    v
                },
            }
        }
    }

    impl<T> AddAssign<&Self> for Matrix<T>
    where
        T: AddAssign + Clone,
    {
        fn add_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::add_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] += rhs.array[i].clone()
            }
        }
    }

    impl<T> SubAssign<&Self> for Matrix<T>
    where
        T: SubAssign + Clone,
    {
        fn sub_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::sub_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] -= rhs.array[i].clone()
            }
        }
    }

    impl<T> BitAndAssign<&Self> for Matrix<T>
    where
        T: BitAndAssign + Clone,
    {
        fn bitand_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitand_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] &= rhs.array[i].clone()
            }
        }
    }

    impl<T> BitOrAssign<&Self> for Matrix<T>
    where
        T: BitOrAssign + Clone,
    {
        fn bitor_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitor_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] |= rhs.array[i].clone()
            }
        }
    }

    impl<T> BitXorAssign<&Self> for Matrix<T>
    where
        T: BitXorAssign + Clone,
    {
        fn bitxor_assign(&mut self, rhs: &Matrix<T>) {
            if !(self.n == rhs.n && self.m == rhs.m) {
                panic!("`Matrix::bitxor_assign` needs two Matrix<T> the same sized.");
            }
            for i in 0..self.n * self.m {
                self.array[i] ^= rhs.array[i].clone()
            }
        }
    }

    impl<T> Shl<usize> for &Matrix<T>
    where
        T: Shl<usize, Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn shl(self, rhs: usize) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() << rhs)
                    }
                    v
                },
            }
        }
    }

    impl<T> Shr<usize> for &Matrix<T>
    where
        T: Shr<usize, Output = T> + Clone,
    {
        type Output = Matrix<T>;
        fn shr(self, rhs: usize) -> Self::Output {
            Matrix {
                n: self.n,
                m: self.m,
                array: {
                    let mut v = Vec::new();
                    for i in 0..self.n * self.m {
                        v.push(self.array[i].clone() >> rhs)
                    }
                    v
                },
            }
        }
    }

    impl<T> Index<usize> for Matrix<T> {
        type Output = T;
        fn index(&self, index: usize) -> &T {
            if !(index < self.n * self.m) {
                panic!(format!("index fail: {} is out of range.", index))
            }
            &self.array[index]
        }
    }

    impl<T> IndexMut<usize> for Matrix<T> {
        fn index_mut(&mut self, index: usize) -> &mut T {
            if !(index < self.n * self.m) {
                panic!(format!("index_mut fail: {} is out of range.", index));
            }
            &mut self.array[index]
        }
    }
}
// TEST
#[cfg(test)]
mod tests {
    use crate::matrix::algebra::*;

    #[test]
    fn test_matrix_new() {
        assert_eq!(
            Matrix::<f32>::new(3, 4),
            Matrix {
                n: 3,
                m: 4,
                array: vec![0.0; 12]
            }
        );
    }

    #[test]
    fn test_matrix_append() {
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
    fn test_matrix_append_panic() {
        let _dummy_matrix = Matrix::append(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_matrix_append_line() {
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
    fn test_matrix_append_line_panic() {
        let _dummy_matrix =
            Matrix::append_line(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8], vec![9]]);
    }

    #[test]
    fn test_matrix_append_column() {
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
    fn test_matrix_append_column_panic() {
        let _dummy_matrix =
            Matrix::append_column(vec![vec![1, 4, 7, 10], vec![2, 5, 8, 11], vec![3, 6, 9]]);
    }

    #[test]
    fn test_matrix_neg() {
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
    fn test_matrix_not() {
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
    fn test_matrix_add() {
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
    fn test_matrix_add_panic() {
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
    fn test_matrix_addassign() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix += &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix
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
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix.add_assign(&Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                });
                dummy_matrix
            },
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
    #[should_panic(expected = "`Matrix::add_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_addassign_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        dummy_matrix += &Matrix {
            n: 4,
            m: 3,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_matrix_sub() {
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
    fn test_matrix_sub_panic() {
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
    fn test_matrix_subassign() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix -= &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix
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
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix.sub_assign(&Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                });
                dummy_matrix
            },
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
    #[should_panic(expected = "`Matrix::sub_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_subassign_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        dummy_matrix -= &Matrix {
            n: 4,
            m: 3,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_matrix_bitand() {
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
    fn test_matrix_bitand_panic() {
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

    #[test]
    fn test_matrix_bitand_assign() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false,
                    ],
                };
                dummy_matrix &= &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true,
                    ],
                };
                dummy_matrix
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
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false,
                    ],
                };
                dummy_matrix.bitand_assign(&Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true,
                    ],
                });
                dummy_matrix
            },
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
    #[should_panic(expected = "`Matrix::bitand_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_bitand_assign_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![
                true, true, false, false, true, false, true, false, false, true, true, false,
            ],
        };
        dummy_matrix &= &Matrix {
            n: 4,
            m: 3,
            array: vec![
                true, false, false, true, true, false, false, true, false, true, false, true,
            ],
        };
    }

    #[test]
    fn test_matrix_bitor() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false
                ]
            } | &Matrix {
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
                    true | true,
                    true | false,
                    false | false,
                    false | true,
                    true | true,
                    false | false,
                    true | false,
                    false | true,
                    false | false,
                    true | true,
                    true | false,
                    false | true
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
            .bitor(&Matrix {
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
                    true.bitor(true),
                    true.bitor(false),
                    false.bitor(false),
                    false.bitor(true),
                    true.bitor(true),
                    false.bitor(false),
                    true.bitor(false),
                    false.bitor(true),
                    false.bitor(false),
                    true.bitor(true),
                    true.bitor(false),
                    false.bitor(true)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::bitor` needs two Matrix<T> the same sized.")]
    fn test_matrix_bitor_panic() {
        let _dummy_matrix = &Matrix {
            n: 3,
            m: 4,
            array: vec![
                true, true, false, false, true, false, true, false, false, true, true, false,
            ],
        } | &Matrix {
            n: 4,
            m: 3,
            array: vec![
                true, false, false, true, true, false, false, true, false, true, false, true,
            ],
        };
    }

    #[test]
    fn test_matrix_bitor_assign() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false,
                    ],
                };
                dummy_matrix |= &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true,
                    ],
                };
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true | true,
                    true | false,
                    false | false,
                    false | true,
                    true | true,
                    false | false,
                    true | false,
                    false | true,
                    false | false,
                    true | true,
                    true | false,
                    false | true
                ]
            }
        );

        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false,
                    ],
                };
                dummy_matrix.bitor_assign(&Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true,
                    ],
                });
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true.bitor(true),
                    true.bitor(false),
                    false.bitor(false),
                    false.bitor(true),
                    true.bitor(true),
                    false.bitor(false),
                    true.bitor(false),
                    false.bitor(true),
                    false.bitor(false),
                    true.bitor(true),
                    true.bitor(false),
                    false.bitor(true)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::bitor_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_bitor_assign_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![
                true, true, false, false, true, false, true, false, false, true, true, false,
            ],
        };
        dummy_matrix |= &Matrix {
            n: 4,
            m: 3,
            array: vec![
                true, false, false, true, true, false, false, true, false, true, false, true,
            ],
        };
    }

    #[test]
    fn test_matrix_bitxor() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true, true, false, false, true, false, true, false, false, true, true, false
                ]
            } ^ &Matrix {
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
                    true ^ true,
                    true ^ false,
                    false ^ false,
                    false ^ true,
                    true ^ true,
                    false ^ false,
                    true ^ false,
                    false ^ true,
                    false ^ false,
                    true ^ true,
                    true ^ false,
                    false ^ true
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
            .bitxor(&Matrix {
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
                    true.bitxor(true),
                    true.bitxor(false),
                    false.bitxor(false),
                    false.bitxor(true),
                    true.bitxor(true),
                    false.bitxor(false),
                    true.bitxor(false),
                    false.bitxor(true),
                    false.bitxor(false),
                    true.bitxor(true),
                    true.bitxor(false),
                    false.bitxor(true)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::bitxor` needs two Matrix<T> the same sized.")]
    fn test_matrix_bitxor_panic() {
        let _dummy_matrix = &Matrix {
            n: 3,
            m: 4,
            array: vec![
                true, true, false, false, true, false, true, false, false, true, true, false,
            ],
        } ^ &Matrix {
            n: 4,
            m: 3,
            array: vec![
                true, false, false, true, true, false, false, true, false, true, false, true,
            ],
        };
    }

    #[test]
    fn test_matrix_bitxor_assign() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false,
                    ],
                };
                dummy_matrix ^= &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true,
                    ],
                };
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true ^ true,
                    true ^ false,
                    false ^ false,
                    false ^ true,
                    true ^ true,
                    false ^ false,
                    true ^ false,
                    false ^ true,
                    false ^ false,
                    true ^ true,
                    true ^ false,
                    false ^ true
                ]
            }
        );

        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, true, false, false, true, false, true, false, false, true, true,
                        false,
                    ],
                };
                dummy_matrix.bitxor_assign(&Matrix {
                    n: 4,
                    m: 3,
                    array: vec![
                        true, false, false, true, true, false, false, true, false, true, false,
                        true,
                    ],
                });
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    true.bitxor(true),
                    true.bitxor(false),
                    false.bitxor(false),
                    false.bitxor(true),
                    true.bitxor(true),
                    false.bitxor(false),
                    true.bitxor(false),
                    false.bitxor(true),
                    false.bitxor(false),
                    true.bitxor(true),
                    true.bitxor(false),
                    false.bitxor(true)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::bitxor_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_bitxor_assign_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![
                true, true, false, false, true, false, true, false, false, true, true, false,
            ],
        };
        dummy_matrix ^= &Matrix {
            n: 4,
            m: 3,
            array: vec![
                true, false, false, true, true, false, false, true, false, true, false, true,
            ],
        };
    }

    #[test]
    fn test_matrix_shl() {
        assert_eq!(
            &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } << 4_usize,
            Matrix {
                n: 3,
                m: 4,
                array: vec![
                    1 << 4,
                    2 << 4,
                    3 << 4,
                    4 << 4,
                    5 << 4,
                    6 << 4,
                    7 << 4,
                    8 << 4,
                    9 << 4,
                    10 << 4,
                    11 << 4,
                    12 << 4
                ]
            }
        );
        assert_eq!(
            Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .shl(4_usize),
            Matrix {
                n: 3,
                m: 4,
                array: vec![
                    1 << 4,
                    2 << 4,
                    3 << 4,
                    4 << 4,
                    5 << 4,
                    6 << 4,
                    7 << 4,
                    8 << 4,
                    9 << 4,
                    10 << 4,
                    11 << 4,
                    12 << 4
                ]
            }
        )
    }

    #[test]
    fn test_matrix_shr() {
        assert_eq!(
            &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } >> 4_usize,
            Matrix {
                n: 3,
                m: 4,
                array: vec![
                    1 >> 4,
                    2 >> 4,
                    3 >> 4,
                    4 >> 4,
                    5 >> 4,
                    6 >> 4,
                    7 >> 4,
                    8 >> 4,
                    9 >> 4,
                    10 >> 4,
                    11 >> 4,
                    12 >> 4
                ]
            }
        );
        assert_eq!(
            Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .shr(4_usize),
            Matrix {
                n: 3,
                m: 4,
                array: vec![
                    1 >> 4,
                    2 >> 4,
                    3 >> 4,
                    4 >> 4,
                    5 >> 4,
                    6 >> 4,
                    7 >> 4,
                    8 >> 4,
                    9 >> 4,
                    10 >> 4,
                    11 >> 4,
                    12 >> 4
                ]
            }
        )
    }

    #[test]
    fn test_matrix_index() {
        let dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        for i in 0..12 {
            assert_eq!(dummy_matrix.index(i), &(i + 1))
        }
    }

    #[test]
    #[should_panic(expected = "index fail: 12 is out of range.")]
    fn test_matrix_index_panic() {
        let dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        for i in 0..13 {
            assert_eq!(dummy_matrix.index(i), &(i + 1))
        }
    }

    #[test]
    fn test_matrix_index_mut() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        for i in 0..12 {
            *dummy_matrix.index_mut(i) -= 1;
        }
        for i in 0..12 {
            assert_eq!(dummy_matrix.index_mut(i), &mut (i as i32))
        }
    }

    #[test]
    #[should_panic(expected = "index_mut fail: 12 is out of range.")]
    fn test_matrix_index_mut_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        for i in 0..12 {
            *dummy_matrix.index_mut(i) -= 1;
        }
        for i in 0..13 {
            assert_eq!(dummy_matrix.index_mut(i), &mut (i as i32))
        }
    }
}

// fn plus() {
//     &algebra::Matrix {
//         n: 3,
//         m: 4,
//         array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
//     } + &algebra::Matrix {
//         n: 3,
//         m: 4,
//         array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
//     };
// }
