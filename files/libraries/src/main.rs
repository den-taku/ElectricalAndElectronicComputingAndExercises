mod bisection_method;
mod data;
mod matrix;
mod newton_raphson_method;

use gnuplot::*;
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
    let data_jacobi = jacobi.solve(10e-10, 10_000);
    // println!("times: {}", times_jacobi);
    println!("{}", jacobi);
    //  println!("{:?}", data_jacobi);

    let mut fg = Figure::new();
    {
        let axec = fg
            .axes2d()
            .set_x_axis(true, &[])
            .set_x_range(Fix(0.0), Fix(4500.0))
            .set_y_range(Fix(1.0e-10), Fix(1.0))
            .set_y_log(Some(10.0))
            .set_x_label("times", &[])
            .set_y_label("residual\\_norm", &[]);
        // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
        // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

        data_jacobi.iter().fold((), |_, e| {
            axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
        })
    }
    let _ = fg.show();

    let mut gauss_seidel =
        GaussSeidel::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    let data_gauss_seidel = gauss_seidel.solve(10e-10, 10_000);
    // println!("times: {}", times_gaussseidel);
    println!("{}", gauss_seidel);

    let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.863);
    let data_sor = sor.solve(10e-10, 10_000);
    // println!("times: {}", times_sor);
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
    // println!("times: {}", times_jacobi);
    println!("{}", jacobi);

    let mut gauss_seidel =
        GaussSeidel::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    let times_gaussseidel = gauss_seidel.solve(10e-10, 10_000);
    // println!("times: {}", times_gaussseidel);
    println!("{}", gauss_seidel);

    let mut sor = SOR::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.863);
    let times_sor = sor.solve(10e-10, 10_000);
    // println!("times: {}", times_sor);
    println!("{}", sor);

    let matrixa = data::matrix23a_f64();
    let matrixb = data::matrix23b_f64();
    println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));

    // let matrixa = data::matrix21a_f64();
    // let matrixb = data::matrix21b_f64();

    // let mut data = Vec::new();
    // for i in 1..2000 {
    //    let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), i as f64 / 1000.0);
    //    let omega = i as f64 / 1000.0;
    //    let times = sor.solve(10e-10, 10_000);
    //    data.push((omega, times));
    //    println!("ω: {}, times: {}", omega, times);
    // }
    // let mut fg = Figure::new();
    // {
    //     let axec = fg
    //         .axes2d()
    //         .set_x_axis(true, &[])
    //         .set_x_range(Fix(0.0), Fix(2.0))
    //         .set_y_range(Fix(0.0), Fix(10000.0))
    //         // .set_y_log(Some(10.0))
    //         .set_x_label("ω", &[])
    //         .set_y_label("times", &[]);
    //         // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //         // .lines(x, y, &[Caption("newton"), Color("blue")]);

    //     data.iter().fold((), |_, e| {
    //         axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //     })
    // }
    // let _ = fg.show();
}
