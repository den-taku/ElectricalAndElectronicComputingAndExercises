#![allow(dead_code)]

// pub mod algebra {
pub use num_traits::{Float, FromPrimitive, One, ToPrimitive, Zero};
pub use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Index, IndexMut, Mul, MulAssign, Neg, Not, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
pub use std::rc::Rc;

pub use std::fmt;
pub use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix<T> {
    n: usize,      // line           [* * * * *]
    m: usize,      // column         [* * * * *] -> n = 3, m = 5
    array: Vec<T>, //                [* * * * *]
}

// pub trait Algebra<T> {
//     fn new(n: usize, m: usize) -> Self;
//     fn append(n: usize, m : usize) -> Self;
//     fn append_line(vec: Vec<Vec<T>>) -> Self;
//     fn append_column(vec: Vec<Vec<T>>) -> Self;
//     fn transpose(&mut self);
//     fn map(&mut self, f: Rc<dyn Fn(T) -> T>) -> Self;
//     fn map_new<R>(&self, f: Rc<dyn Fn(T) -> R>) -> Self<R>;
//     fn norm2<F>(&self) -> F;
//     fn is_square(&self) -> bool;
//     fn lu_decompose(&self) -> (Self, Self);
// }

impl Matrix<f64> {
    pub fn forward_erase(a: &Self, b: &Self) -> Self {
        let a = a.clone();
        let b = b.clone();
        if a.m != a.n || a.n != b.n || b.m != 1 {
            panic!("`Matrix::forward_erase` needs apropriate matrix.")
        }
        let mut v_a = vec![vec![]; a.n];
        println!("1");
        for i in 0..a.n {
            for j in 0..a.m {
                v_a[i].push(a[i * a.m + j].clone())
            }
        }
        // println!("Test");
        // println!("{:?}", v_a);
        println!("2");
        // println!("{}", v_a[0].len());
        for i in 0..a.n {
            v_a[i].push(b[i]);
        }
        // println!("{:?}", v_a);
        // let a00 = v_a[0][0];
        // println!("{}", v_a[0].len());
        // println!("3");
        // for i in 0..a.m+1 {
        //     v_a[0][i] /= a00;
        // }
        println!("4");
        for i in 0..a.n {
            // let v_const = v_a[i-1].clone();
            let index = {
                let mut v_tmp = Vec::new();
                println!("5");
                for j in i..a.m {  
                    v_tmp.push((v_a[j][i].clone(), j));
                }
                v_tmp.sort_by(|a, b| a.partial_cmp(b).unwrap());
                v_tmp.pop().unwrap().1
            };
            v_a.swap(i, index);
            let a0 = v_a[i][i];
            println!("v_a[i][0]: {}", a0);
            println!("6");
            for j in i..a.m+1 {
                v_a[i][j] /= a0;
            }
            println!("7");
            for k in i+1..a.n {
                let c = v_a[k][i].clone();
                println!("c: {}", c);
                println!("8");
                for l in i..a.m+1 {
                    // println!("v_a[k]'s size: {}", v_a[k].len());
                    // println!("v_a[i]'s size: {}", v_a[i].len());
                    // println!("i: {}, l: {}", i, l);
                    v_a[k][l] -= c * v_a[i][l];
                }
                // println!("k: {}", k);
                println!("9");
            }
        }
        Matrix::append_line(v_a)
    }

    pub fn backward_erase(ab: Self) -> Self {
        //
    }
}

impl<T> Matrix<T>
where
    T: Clone + Zero,
{
    pub fn new(n: usize, m: usize) -> Self {
        Matrix {
            n,
            m,
            array: vec![T::zero(); n * m],
        }
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
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

    pub fn transpose(&mut self) {
        let mut new_array = Vec::new();
        for j in 0..self.m {
            for i in 0..self.n {
                new_array.push(self.array[i * self.m + j].clone());
            }
        }
        self.array = new_array;
        let tmp = self.n;
        self.n = self.m;
        self.m = tmp;
    }

    pub fn map(&mut self, f: Rc<dyn Fn(T) -> T>) -> Self {
        for i in 0..self.n * self.m {
            self.array[i] = f(self.array[i].clone())
        }
        self.clone()
    }

    pub fn map_new<R>(&self, f: Rc<dyn Fn(T) -> R>) -> Matrix<R> {
        let mut mapped_array = Vec::new();
        for i in 0..self.n * self.m {
            mapped_array.push(f(self.array[i].clone()))
        }
        Matrix {
            n: self.n,
            m: self.m,
            array: mapped_array,
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.array.clone()
    }
}

impl<T> Matrix<T>
where
    T: Zero
        + Clone
        + ToPrimitive
        + One
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>,
{
    pub fn norm2<F>(&self) -> F
    where
        F: Float + Zero + FromPrimitive + Add<Output = F>,
    {
        let mut size = F::zero();
        for i in 0..self.n * self.m {
            size = size.clone()
                + F::from(self.array[i].clone())
                    .unwrap()
                    .powf(F::from_f32(2.0).unwrap())
        }
        size.sqrt()
    }

    pub fn is_square(&self) -> bool {
        self.n == self.m
    }

    pub fn lu_decompose(&self) -> (Matrix<T>, Matrix<T>) {
        // use Crout method
        if !self.is_square() {
            panic!("`Matrix::lu_decompose` needs square matrix");
        }
        let mut l = vec![vec![T::zero(); self.n]; self.n];
        let mut u = vec![vec![T::zero(); self.n]; self.n];
        for i in 0..self.n {
            l[i][i] = T::one();
        }
        let mut dec = self.array.clone();

        for j in 0..self.n - 1 {
            let w = T::one() / dec[j * self.n + j].clone();
            for i in j + 1..self.n {
                dec[i * self.n + j] = w.clone() * dec[i * self.n + j].clone();
                for k in j + 1..self.n {
                    dec[i * self.n + k] = dec[i * self.n + k].clone()
                        - dec[i * self.n + j].clone() * dec[j * self.n + k].clone();
                }
            }
        }

        for j in 0..self.n {
            for i in 0..j + 1 {
                u[i][j] = dec[i * self.n + j].clone();
            }
            for i in j + 1..self.n {
                l[i][j] = dec[i * self.n + j].clone()
            }
        }
        (Matrix::append_line(l), Matrix::append_line(u)) //TODO
    }
}

impl<F> Matrix<F>
where
    F: Float,
{
    pub fn solve_eqn(a: &Self, b: &Self) -> Self {
        if !(a.is_square() && a.n == b.n) {
            panic!("`Matrix::solve_eqn` needs n * n matrix and n vector.");
        }
        let mut b_mut = b.clone();
        let lu = a.lu_decompose();
        // let y = Vec::new();
        // y.push(b_mut[0] / lu.0[0]); // TODO
        for i in 0..a.n - 1 {
            for j in i + 1..a.n {
                b_mut[j] = b_mut[j].clone() - lu.0[j * a.n + i].clone() * b_mut[i].clone()
            }
        }
        for i in (0..a.n).rev() {
            b_mut[i] = b_mut[i].clone() / lu.1[i * a.n + i].clone();
            for k in (0..i).rev() {
                b_mut[k] = b_mut[k].clone() - lu.1[k * a.n + i].clone() * b_mut[i].clone();
            }
        }
        b_mut.m = 1usize;
        b_mut.n = b_mut.array.len();
        b_mut
    }
}

impl<R, T> Matrix<Rc<dyn Fn(R) -> T>>
where
    R: Clone,
{
    pub fn applicate(&self, x: &Vec<R>) -> Matrix<T> {
        if !(self.n * self.m == x.len()) {
            panic!(format!(
                "Matrix<R>::applicate needs {} elements",
                self.n * self.m
            ));
        }
        let mut mapped_array = Vec::new();
        for i in 0..self.n * self.m {
            mapped_array.push(self.array[i](x[i].clone()))
        }
        Matrix {
            n: self.n,
            m: self.m,
            array: mapped_array,
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

impl<T> Add<Self> for &Matrix<T>
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

impl<T> Add<T> for &Matrix<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output {
        Matrix {
            n: self.n,
            m: self.m,
            array: {
                let mut v = Vec::new();
                for i in 0..self.n * self.m {
                    v.push(self.array[i].clone() + rhs.clone());
                }
                v
            },
        }
    }
}

impl<T> Sub<Self> for &Matrix<T>
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

impl<T> Sub<T> for &Matrix<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Matrix {
            n: self.n,
            m: self.m,
            array: {
                let mut v = Vec::new();
                for i in 0..self.n * self.m {
                    v.push(self.array[i].clone() - rhs.clone());
                }
                v
            },
        }
    }
}

impl<T> Mul<Self> for &Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone + Zero,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        // TODO: use Strassen algorithm
        if !(self.m == rhs.n) {
            panic!("`Matrix::mul` needs n * m Matrix<T> and m * k Matrix<T>.")
        }
        Matrix {
            n: self.n,
            m: rhs.m,
            array: {
                let mut v = Vec::<T>::new();
                for i in 0..self.n {
                    for j in 0..rhs.m {
                        let mut sum = T::zero();
                        for k in 0..self.m {
                            sum = sum
                                + self.array[i * self.m + k].clone()
                                    * rhs.array[j + k * rhs.m].clone()
                        }
                        v.push(sum)
                    }
                }
                v
            },
        }
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Matrix {
            n: self.n,
            m: self.m,
            array: {
                let mut v = Vec::new();
                for i in 0..self.n * self.m {
                    v.push(self.array[i].clone() * rhs.clone())
                }
                v
            },
        }
    }
}

impl<T> Div<T> for &Matrix<T>
where
    T: Div<Output = T> + Clone,
{
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Self::Output {
        Matrix {
            n: self.n,
            m: self.m,
            array: {
                let mut v = Vec::new();
                for i in 0..self.n * self.m {
                    v.push(self.array[i].clone() / rhs.clone())
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

impl<T> AddAssign<T> for Matrix<T>
where
    T: AddAssign + Clone,
{
    fn add_assign(&mut self, rhs: T) {
        for i in 0..self.n * self.m {
            self.array[i] += rhs.clone()
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

impl<T> SubAssign<T> for Matrix<T>
where
    T: SubAssign + Clone,
{
    fn sub_assign(&mut self, rhs: T) {
        for i in 0..self.n * self.m {
            self.array[i] -= rhs.clone()
        }
    }
}

impl<T> MulAssign<&Self> for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Clone + Zero,
{
    fn mul_assign(&mut self, rhs: &Matrix<T>) {
        if !(self.m == rhs.n) {
            panic!("`Matrix::mul_assign` needs n * m Matrix<T> and m * k Matrix<T>.");
        }
        let mut v = Vec::<T>::new();
        for i in 0..self.n {
            for j in 0..rhs.m {
                let mut sum = T::zero();
                for k in 0..self.m {
                    sum =
                        sum + self.array[i * self.m + k].clone() * rhs.array[j + k * rhs.m].clone()
                }
                v.push(sum)
            }
        }
        self.m = rhs.m;
        self.array = v;
    }
}

impl<T> MulAssign<T> for Matrix<T>
where
    T: MulAssign<T> + Clone,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..self.n * self.m {
            self.array[i] *= rhs.clone()
        }
    }
}

impl<T> DivAssign<T> for Matrix<T>
where
    T: DivAssign<T> + Clone,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..self.n * self.m {
            self.array[i] /= rhs.clone()
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

impl<T> Display for Matrix<T>
where
    T: Display + Clone + PartialOrd + Zero,
{
    fn fmt(&self, dest: &mut Formatter) -> fmt::Result {
        let mut string = "".to_string();
        for i in 0..self.n {
            for j in 0..self.m {
                let pad = if self[i * self.m + j] >= T::zero() {
                    " ".to_string()
                } else {
                    "".to_string()
                };
                string = format!("{}{}{} ", string, pad, self[i * self.m + j].clone());
            }
            string = format!("{}\n", string);
        }
        write!(dest, "{}", string)
    }
}
// TEST
#[cfg(test)]
mod tests_matrix {
    use crate::matrix::Matrix;
    use crate::matrix::*;

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
    fn test_matrix_transpose() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        dummy_matrix.transpose();
        assert_eq!(
            dummy_matrix,
            Matrix {
                n: 4,
                m: 3,
                array: vec![1, 5, 9, 2, 6, 10, 3, 7, 11, 4, 8, 12]
            }
        );
        dummy_matrix.transpose();
        assert_eq!(
            dummy_matrix,
            Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
        )
    }

    #[test]
    fn test_matrix_map() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        let f: Rc<dyn Fn(i32) -> i32> = Rc::new(|x| x * x);
        assert_eq!(
            dummy_matrix.map(f),
            Matrix {
                n: 3,
                m: 4,
                array: vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144]
            }
        );
    }

    #[test]
    fn test_matrix_map_new() {
        let dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        let f: Rc<fn(i32) -> f32> = Rc::new(|x| x as f32);
        assert_eq!(
            dummy_matrix.map_new(f),
            Matrix {
                n: 3,
                m: 4,
                array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
            }
        )
    }

    #[test]
    fn test_matrix_applicate() {
        let f: Rc<dyn Fn(i32) -> f32> = Rc::new(|x| x as f32);
        let matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![f.clone(); 3 * 4],
        };
        assert_eq!(
            matrix.applicate(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]),
            Matrix {
                n: 3,
                m: 4,
                array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
            }
        );
    }

    #[test]
    #[should_panic(expected = "Matrix<R>::applicate needs 12 elements")]
    fn test_matrix_applicate_panic() {
        let f: Rc<dyn Fn(i32) -> f32> = Rc::new(|x| x as f32);
        let _ = Matrix {
            n: 3,
            m: 4,
            array: vec![f.clone(); 12],
        }
        .applicate(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn test_matrix_norm2() {
        assert_eq!(
            Matrix {
                n: 1,
                m: 2,
                array: vec![3.0, 4.0]
            }
            .norm2::<f32>(),
            5.0
        )
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
    fn test_matrix_add_self() {
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
    fn test_matrix_add_t() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } + 8,
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 + 8,
                    2 + 8,
                    3 + 8,
                    4 + 8,
                    5 + 8,
                    6 + 8,
                    7 + 8,
                    8 + 8,
                    9 + 8,
                    10 + 8,
                    11 + 8,
                    12 + 8
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .add(8)),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.add(8),
                    2.add(8),
                    3.add(8),
                    4.add(8),
                    5.add(8),
                    6.add(8),
                    7.add(8),
                    8.add(8),
                    9.add(8),
                    10.add(8),
                    11.add(8),
                    12.add(8)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::add` needs two Matrix<T> the same sized.")]
    fn test_matrix_add_self_panic() {
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
    fn test_matrix_addassign_self() {
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
    fn test_matrix_addassign_t() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix += 8;
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 + 8,
                    2 + 8,
                    3 + 8,
                    4 + 8,
                    5 + 8,
                    6 + 8,
                    7 + 8,
                    8 + 8,
                    9 + 8,
                    10 + 8,
                    11 + 8,
                    12 + 8
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
                dummy_matrix.add_assign(8);
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.add(8),
                    2.add(8),
                    3.add(8),
                    4.add(8),
                    5.add(8),
                    6.add(8),
                    7.add(8),
                    8.add(8),
                    9.add(8),
                    10.add(8),
                    11.add(8),
                    12.add(8)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::add_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_addassign_self_panic() {
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
    fn test_matrix_sub_self() {
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
    fn test_matrix_sub_t() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } - 8,
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 - 8,
                    2 - 8,
                    3 - 8,
                    4 - 8,
                    5 - 8,
                    6 - 8,
                    7 - 8,
                    8 - 8,
                    9 - 8,
                    10 - 8,
                    11 - 8,
                    12 - 8
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .sub(8)),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.sub(8),
                    2.sub(8),
                    3.sub(8),
                    4.sub(8),
                    5.sub(8),
                    6.sub(8),
                    7.sub(8),
                    8.sub(8),
                    9.sub(8),
                    10.sub(8),
                    11.sub(8),
                    12.sub(8)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::sub` needs two Matrix<T> the same sized.")]
    fn test_matrix_sub_self_panic() {
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
    fn test_matrix_subassign_self() {
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
    fn test_matrix_subassign_t() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix -= 8;
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 - 8,
                    2 - 8,
                    3 - 8,
                    4 - 8,
                    5 - 8,
                    6 - 8,
                    7 - 8,
                    8 - 8,
                    9 - 8,
                    10 - 8,
                    11 - 8,
                    12 - 8
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
                dummy_matrix.sub_assign(8);
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.sub(8),
                    2.sub(8),
                    3.sub(8),
                    4.sub(8),
                    5.sub(8),
                    6.sub(8),
                    7.sub(8),
                    8.sub(8),
                    9.sub(8),
                    10.sub(8),
                    11.sub(8),
                    12.sub(8)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::sub_assign` needs two Matrix<T> the same sized.")]
    fn test_matrix_subassign_self_panic() {
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
    fn test_matrix_mul_self() {
        assert_eq!(
            &Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } * &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            },
            Matrix {
                n: 3,
                m: 3,
                array: vec![
                    1 * 1 + 2 * 4 + 3 * 7 + 4 * 10,
                    1 * 2 + 2 * 5 + 3 * 8 + 4 * 11,
                    1 * 3 + 2 * 6 + 3 * 9 + 4 * 12,
                    5 * 1 + 6 * 4 + 7 * 7 + 8 * 10,
                    5 * 2 + 6 * 5 + 7 * 8 + 8 * 11,
                    5 * 3 + 6 * 6 + 7 * 9 + 8 * 12,
                    9 * 1 + 10 * 4 + 11 * 7 + 12 * 10,
                    9 * 2 + 10 * 5 + 11 * 8 + 12 * 11,
                    9 * 3 + 10 * 6 + 11 * 9 + 12 * 12,
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 3,
                m: 4,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .mul(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            })),
            Matrix {
                n: 3,
                m: 3,
                array: vec![
                    1.mul(1) + 2.mul(4) + 3.mul(7) + 4.mul(10),
                    1.mul(2) + 2.mul(5) + 3.mul(8) + 4.mul(11),
                    1.mul(3) + 2.mul(6) + 3.mul(9) + 4.mul(12),
                    5.mul(1) + 6.mul(4) + 7.mul(7) + 8.mul(10),
                    5.mul(2) + 6.mul(5) + 7.mul(8) + 8.mul(11),
                    5.mul(3) + 6.mul(6) + 7.mul(9) + 8.mul(12),
                    9.mul(1) + 10.mul(4) + 11.mul(7) + 12.mul(10),
                    9.mul(2) + 10.mul(5) + 11.mul(8) + 12.mul(11),
                    9.mul(3) + 10.mul(6) + 11.mul(9) + 12.mul(12),
                ]
            }
        );
    }

    #[test]
    fn test_matrix_mul_t() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            } * 8,
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 * 8,
                    2 * 8,
                    3 * 8,
                    4 * 8,
                    5 * 8,
                    6 * 8,
                    7 * 8,
                    8 * 8,
                    9 * 8,
                    10 * 8,
                    11 * 8,
                    12 * 8
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
            }
            .mul(8)),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.mul(8),
                    2.mul(8),
                    3.mul(8),
                    4.mul(8),
                    5.mul(8),
                    6.mul(8),
                    7.mul(8),
                    8.mul(8),
                    9.mul(8),
                    10.mul(8),
                    11.mul(8),
                    12.mul(8)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::mul` needs n * m Matrix<T> and m * k Matrix<T>.")]
    fn test_matrix_mul_self_panic() {
        let _dummy_matrix = &Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        } * &Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_matrix_mulassign_self() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix *= &Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix
            },
            Matrix {
                n: 3,
                m: 3,
                array: vec![
                    1 * 1 + 2 * 4 + 3 * 7 + 4 * 10,
                    1 * 2 + 2 * 5 + 3 * 8 + 4 * 11,
                    1 * 3 + 2 * 6 + 3 * 9 + 4 * 12,
                    5 * 1 + 6 * 4 + 7 * 7 + 8 * 10,
                    5 * 2 + 6 * 5 + 7 * 8 + 8 * 11,
                    5 * 3 + 6 * 6 + 7 * 9 + 8 * 12,
                    9 * 1 + 10 * 4 + 11 * 7 + 12 * 10,
                    9 * 2 + 10 * 5 + 11 * 8 + 12 * 11,
                    9 * 3 + 10 * 6 + 11 * 9 + 12 * 12,
                ]
            }
        );

        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 3,
                    m: 4,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix.mul_assign(&Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                });
                dummy_matrix
            },
            Matrix {
                n: 3,
                m: 3,
                array: vec![
                    1.mul(1) + 2.mul(4) + 3.mul(7) + 4.mul(10),
                    1.mul(2) + 2.mul(5) + 3.mul(8) + 4.mul(11),
                    1.mul(3) + 2.mul(6) + 3.mul(9) + 4.mul(12),
                    5.mul(1) + 6.mul(4) + 7.mul(7) + 8.mul(10),
                    5.mul(2) + 6.mul(5) + 7.mul(8) + 8.mul(11),
                    5.mul(3) + 6.mul(6) + 7.mul(9) + 8.mul(12),
                    9.mul(1) + 10.mul(4) + 11.mul(7) + 12.mul(10),
                    9.mul(2) + 10.mul(5) + 11.mul(8) + 12.mul(11),
                    9.mul(3) + 10.mul(6) + 11.mul(9) + 12.mul(12),
                ]
            }
        );
    }

    #[test]
    fn test_matrix_mulassign_t() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                };
                dummy_matrix *= 8;
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1 * 8,
                    2 * 8,
                    3 * 8,
                    4 * 8,
                    5 * 8,
                    6 * 8,
                    7 * 8,
                    8 * 8,
                    9 * 8,
                    10 * 8,
                    11 * 8,
                    12 * 8
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
                dummy_matrix.mul_assign(8);
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.mul(8),
                    2.mul(8),
                    3.mul(8),
                    4.mul(8),
                    5.mul(8),
                    6.mul(8),
                    7.mul(8),
                    8.mul(8),
                    9.mul(8),
                    10.mul(8),
                    11.mul(8),
                    12.mul(8)
                ]
            }
        );
    }

    #[test]
    #[should_panic(expected = "`Matrix::mul_assign` needs n * m Matrix<T> and m * k Matrix<T>.")]
    fn test_matrix_mulassgin_self_panic() {
        let mut dummy_matrix = Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
        dummy_matrix *= &Matrix {
            n: 3,
            m: 4,
            array: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
        };
    }

    #[test]
    fn test_matrix_div_t() {
        assert_eq!(
            &Matrix {
                n: 4,
                m: 3,
                array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
            } / 8.,
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1. / 8.,
                    2. / 8.,
                    3. / 8.,
                    4. / 8.,
                    5. / 8.,
                    6. / 8.,
                    7. / 8.,
                    8. / 8.,
                    9. / 8.,
                    10. / 8.,
                    11. / 8.,
                    12. / 8.
                ]
            }
        );

        assert_eq!(
            *(&Matrix {
                n: 4,
                m: 3,
                array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.]
            }
            .div(8.)),
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.0.div(8.),
                    2.0.div(8.),
                    3.0.div(8.),
                    4.0.div(8.),
                    5.0.div(8.),
                    6.0.div(8.),
                    7.0.div(8.),
                    8.0.div(8.),
                    9.0.div(8.),
                    10.0.div(8.),
                    11.0.div(8.),
                    12.0.div(8.)
                ]
            }
        );
    }

    #[test]
    fn test_matrix_divassign_t() {
        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
                };
                dummy_matrix /= 8.;
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1. / 8.,
                    2. / 8.,
                    3. / 8.,
                    4. / 8.,
                    5. / 8.,
                    6. / 8.,
                    7. / 8.,
                    8. / 8.,
                    9. / 8.,
                    10. / 8.,
                    11. / 8.,
                    12. / 8.
                ]
            }
        );

        assert_eq!(
            {
                let mut dummy_matrix = Matrix {
                    n: 4,
                    m: 3,
                    array: vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12.],
                };
                dummy_matrix.div_assign(8.);
                dummy_matrix
            },
            Matrix {
                n: 4,
                m: 3,
                array: vec![
                    1.0.div(8.),
                    2.0.div(8.),
                    3.0.div(8.),
                    4.0.div(8.),
                    5.0.div(8.),
                    6.0.div(8.),
                    7.0.div(8.),
                    8.0.div(8.),
                    9.0.div(8.),
                    10.0.div(8.),
                    11.0.div(8.),
                    12.0.div(8.)
                ]
            }
        );
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

    #[test]
    fn test_matrix_is_square() {
        assert_eq!(
            Matrix {
                n: 2,
                m: 2,
                array: vec![9.0; 4]
            }
            .is_square(),
            true
        );
        assert_eq!(
            Matrix {
                n: 2,
                m: 3,
                array: vec![3; 6]
            }
            .is_square(),
            false
        );
    }

    #[test]
    fn test_matrix_lu_decompose() {
        let a = Matrix::append_line(vec![
            vec![2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![-1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0, 0.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0, -1.0],
            vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 2.0],
        ]);
        let lu = a.lu_decompose();
        assert_eq!(a, &lu.0 * &lu.1);
    }

    #[test]
    #[should_panic(expected = "`Matrix::lu_decompose` needs square matrix")]
    fn test_matrix_lu_decompose_panic() {
        Matrix {
            n: 2,
            m: 3,
            array: vec![0; 6],
        }
        .lu_decompose();
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
