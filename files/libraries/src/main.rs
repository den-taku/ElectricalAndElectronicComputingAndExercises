mod bisection_method;
mod matrix;
mod newton_raphson_method;
mod data;

// use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;
use matrix::*;

// use crate::matrix::*;

fn main() {
   let matrixa = data::matrix21a_f64();
   let matrixb = data::matrix21b_f64();
   // println!("{}", matrixa);
   // println!("{}", matrixb);
   // let ab = Matrix::forward_erase(&matrixa, &matrixb);
   // println!("{}", ab.clone());
   // println!("{}", Matrix::backward_erase(ab));
   println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
   println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));
   // println!("{}", matrixa.lower_triangular_matrix());
   // println!("{}", matrixa.upper_triangular_matrix());
   // println!("{}", matrixa.diagonal_matrix());
   
   let mut jacobi = Jacobi::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
   let times_jacobi = jacobi.solve(10e-10, 10_000);
   println!("times: {}", times_jacobi);
   println!("{}", jacobi);

   let mut gauss_seidel = GaussSeidel::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
   let times_gaussseidel = gauss_seidel.solve(10e-10, 10_000);
   println!("times: {}", times_gaussseidel);
   println!("{}", gauss_seidel);

   let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.2);
   let times_sor = sor.solve(10e-10, 10_000);
   println!("times: {}", times_sor);
   println!("{}", sor);

   let matrixa = data::matrix21a_f32();
   let matrixb = data::matrix21b_f32();
   // println!("{}", matrixa);
   // println!("{}", matrixb);
   // let ab = Matrix::forward_erase(&matrixa, &matrixb);
   // println!("{}", ab.clone());
   // println!("{}", Matrix::backward_erase(ab));
   // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
   // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));
   // println!("{}", matrixa.lower_triangular_matrix());
   // println!("{}", matrixa.upper_triangular_matrix());
   // println!("{}", matrixa.diagonal_matrix());
   
   let mut jacobi = Jacobi::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
   let times_jacobi = jacobi.solve(10e-10, 10_000);
   println!("times: {}", times_jacobi);
   println!("{}", jacobi);

   let mut gauss_seidel = GaussSeidel::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
   let times_gaussseidel = gauss_seidel.solve(10e-10, 10_000);
   println!("times: {}", times_gaussseidel);
   println!("{}", gauss_seidel);

   let mut sor = SOR::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.2);
   let times_sor = sor.solve(10e-10, 10_000);
   println!("times: {}", times_sor);
   println!("{}", sor);
}
