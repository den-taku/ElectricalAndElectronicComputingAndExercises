use crate::matrix::*;

pub fn matrix21a() -> Matrix<f64> {
    let l1 = vec![12f64, 1., 5., 1., 1., 2., -4., 1., 2.];
    let l2 = vec![1f64, 16., -1., -4., -5., -2., 1., 2., 3.];
    let l3 = vec![5f64, -1., 15., -5., 3., 1., -2., 1., -4.];
    let l4 = vec![1f64, -4., -5., 10., 3., -3., -1., 4., 1.];
    let l5 = vec![1f64, -5., 3., 3., 11., -1., 4., 1., 1.];
    let l6 = vec![2f64, -2., 1., -3., -1., 15., -5., 2., 5.];
    let l7 = vec![-4f64, 1., -2., -1., 4., -5., 15., 4., -4.];
    let l8 = vec![1f64, 2., 1., 4., 1., 2., 4., 11., -1.];
    let l9 = vec![2f64, 3., -4., 1., 1., 5., -4., -1., 15.];
    Matrix::append_line(vec![l1, l2, l3, l4, l5, l6, l7, l8, l9])
}

pub fn matrix21b() -> Matrix<f64> {
    Matrix::append_column(vec![vec![21f64, 11., 13., 6., 18., 14., 8., 25., 18.]])
}