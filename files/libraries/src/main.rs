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
   let matrixa = data::matrix21a();
   let matrixb = data::matrix21b();
   println!("{}", matrixa);
   println!("{}", matrixb);
   // let ab = Matrix::forward_erase(&matrixa, &matrixb);
   // println!("{}", ab.clone());
   // println!("{}", Matrix::backward_erase(ab));
   println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
   println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));
   // 
}
