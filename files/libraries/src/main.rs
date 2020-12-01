mod bisection_method;
mod data;
mod matrix;
mod newton_raphson_method;

// use gnuplot::*;
// use std::rc::Rc;
// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::style::{PointMarker, PointStyle, LineStyle, LineJoin};
// use plotlib::view::ContinuousView;
use matrix::*;

// use crate::matrix::*;

fn main() {
    // let matrixa = data::matrix21a_f64();
    // let matrixb = data::matrix21b_f64();
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

    // let mut jacobi = Jacobi::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_jacobi = jacobi.solve(10e-10, 10_000);
    // // println!("times: {}", times_jacobi);
    // println!("{}", jacobi);
    // println!("{:?}", data_jacobi.pop().unwrap());
    //  println!("{:?}", data_jacobi);

    //  let mut fg = Figure::new();
    //  {
    //      let axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(4500.0))
    //          .set_y_range(Fix(1.0e-10), Fix(1.0))
    //          .set_y_log(Some(10.0))
    //          .set_x_label("times", &[])
    //          .set_y_label("residual\\_norm", &[]);
    //      // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //      // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

    //      data_jacobi.iter().fold((), |_, e| {
    //          axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //      })
    //  }
    //  let _ = fg.show();

    // let mut gauss_seidel =
    //     GaussSeidel::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_gauss_seidel = gauss_seidel.solve(10e-10, 10_000);
    // // println!("times: {}", times_gaussseidel);
    // println!("{}", gauss_seidel);
    // println!("{:?}", data_gauss_seidel.pop().unwrap());

    //  let mut fg = Figure::new();
    //  {
    //      let axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(2000.0))
    //          .set_y_range(Fix(1.0e-10), Fix(1.0))
    //          .set_y_log(Some(10.0))
    //          .set_x_label("times", &[])
    //          .set_y_label("residual\\_norm", &[]);
    //      // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //      // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

    //      data_gauss_seidel.iter().fold((), |_, e| {
    //          axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //      })
    //  }
    //  let _ = fg.show();

    // let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.863);
    // let mut data_sor = sor.solve(10e-10, 10_000);
    // // println!("times: {}", times_sor);
    // println!("{}", sor);
    // println!("{:?}", data_sor.pop().unwrap());

    // println!(
    //     "Jacobi:       {}",
    //     (&jacobi.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f64>()
    // );
    // println!(
    //     "Gauss-Seidek: {}",
    //     (&gauss_seidel.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f64>()
    // );
    // println!(
    //     "SOR:          {}",
    //     (&sor.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f64>()
    // );

    // println!("");

    //     let mut fg = Figure::new();

    //     let x_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.0).collect();
    //     let y_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.1).collect();
    //     let x_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.0).collect();
    //     let y_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.1).collect();
    //     let x_sor: Vec<f64> = data_sor.iter().map(|e| e.0).collect();
    //     let y_sor: Vec<f64> = data_sor.iter().map(|e| e.1).collect();
    //     {
    //         let _axec = fg
    //             .axes2d()
    //             .set_x_axis(true, &[])
    //             .set_x_range(Fix(0.0), Fix(4500.0))
    //             .set_y_range(Fix(10e-10), Fix(1.0))
    //             .set_y_log(Some(10.0))
    //             .set_x_label("times", &[])
    //             .set_y_label("residual\\_norm", &[])
    //         // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //         .lines(x_jacobi, y_jacobi, &[Caption("Jacobi"), Color("green")])
    //         .lines(x_gause_seidel, y_gause_seidel, &[Caption("GaussSeidel"), Color("red")])
    //         .lines(x_sor, y_sor, &[Caption("SOR"), Color("blue")]);

    //       //   data_sor.iter().fold((), |_, e| {
    //       //       axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //       //   });
    //       //   data_gauss_seidel.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("red"), PointSymbol('O')]);
    //       //   });
    //       //   data_jacobi.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("green"), PointSymbol('O')]);
    // //   });
    //     }
    //     let _ = fg.show();

    // println!("ZZZZZZZZZZZZ f32 ZZZZZZZZZZZZZzz");

    // let matrixa = data::matrix21a_f32();
    // let matrixb = data::matrix21b_f32();
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

    // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));

    // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));

    // let mut jacobi = Jacobi::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_jacobi = jacobi.solve(10e-7, 10_000);
    // // println!("times: {}", times_jacobi);
    // println!("{}", jacobi);
    // println!("{:?}", data_jacobi.pop().unwrap());

    // let mut gauss_seidel =
    //     GaussSeidel::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_gaussseidel = gauss_seidel.solve(10e-7, 10_000);
    // // println!("times: {}", times_gaussseidel);
    // println!("{}", gauss_seidel);
    // println!("{:?}", data_gaussseidel.pop().unwrap());

    // println!("KOKO?");
    // let mut sor = SOR::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.84);
    // let mut data_sor = sor.solve(10e-7, 10_000);
    // // println!("times: {}", times_sor);
    // println!("{}", sor);
    // println!("{:?}", data_sor.pop().unwrap());

    // println!(
    //     "Jacobi:       {}",
    //     (&jacobi.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f32>()
    // );
    // println!(
    //     "Gauss-Seidek: {}",
    //     (&gauss_seidel.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f32>()
    // );
    // println!(
    //     "SOR:          {}",
    //     (&sor.approximate_answer() - &Matrix::append(9, 1, vec![1.0; 9])).norm2::<f32>()
    // );
    //     let mut fg = Figure::new();

    //     let x_jacobi: Vec<f32> = data_jacobi.iter().map(|e| e.0).collect();
    //     let y_jacobi: Vec<f32> = data_jacobi.iter().map(|e| e.1).collect();
    //     let x_gause_seidel: Vec<f32> = data_gaussseidel.iter().map(|e| e.0).collect();
    //     let y_gause_seidel: Vec<f32> = data_gaussseidel.iter().map(|e| e.1).collect();
    //     let x_sor: Vec<f32> = data_sor.iter().map(|e| e.0).collect();
    //     let y_sor: Vec<f32> = data_sor.iter().map(|e| e.1).collect();
    //     {
    //         let _axec = fg
    //             .axes2d()
    //             .set_x_axis(true, &[])
    //             .set_x_range(Fix(0.0), Fix(2200.0))
    //             .set_y_range(Fix(10e-10), Fix(1.0))
    //             .set_y_log(Some(10.0))
    //             .set_x_label("times", &[])
    //             .set_y_label("residual\\_norm", &[])
    //         // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //         .lines(x_jacobi, y_jacobi, &[Caption("Jacobi"), Color("green")])
    //         .lines(x_gause_seidel, y_gause_seidel, &[Caption("GaussSeidel"), Color("red")])
    //         .lines(x_sor, y_sor, &[Caption("SOR"), Color("blue")]);

    //       //   data_sor.iter().fold((), |_, e| {
    //       //       axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //       //   });
    //       //   data_gauss_seidel.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("red"), PointSymbol('O')]);
    //       //   });
    //       //   data_jacobi.iter().fold((), |_, e| {
    //       //    axec.points(&[e.0], &[e.1], &[Color("green"), PointSymbol('O')]);
    // //   });
    //     }
    //     let _ = fg.show();

    //  let mut data = Vec::new();
    // for i in 1..200 {
    //    let mut sor = SOR::<f32>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), i as f32 / 100.0);
    //    let mut data_sor = sor.solve(10e-7, 10_000);
    //    let times = data_sor.pop().unwrap().0;
    //    data.push((i as f64 / 100.0,  times));
    //    println!("ω: {}, times: {}", i as f64 / 100.0, times);
    //    //
    // }

    //  let mut fg = Figure::new();
    //  {
    //      let axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(2.0))
    //          .set_y_range(Fix(0.0), Fix(10000.0))
    //          // .set_y_log(Some(10.0))
    //          .set_x_label("relaxation\\_parameter", &[])
    //          .set_y_label("times", &[]);
    //      // .set_y_ticks(Some((Fix(-12.0), 1)), &[], &[])
    //      // .lines(x, y, &[Caption("residual_norm"), Color("blue")]);

    //      data.iter().fold((), |_, e| {
    //          axec.points(&[e.0], &[e.1], &[Color("blue"), PointSymbol('O')]);
    //      })
    //  }
    //  let _ = fg.show();

    //  let matrixa = data::matrix23a_f64();
    //  let matrixb = data::matrix23b_f64();
    //  println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));

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

    // let matrixa = data::matrix23a_f64();
    // let matrixb = data::matrix23b_f64();
    // println!("{}", Matrix::solve_eqn_gauss(&matrixa, &matrixb));
    // println!("{}", Matrix::solve_eqn(&matrixa, &matrixb));

    // let mut jacobi = Jacobi::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_jacobi = jacobi.solve(10e-10, 10_000);
    // println!("{}", jacobi);
    // println!("{:?}", data_jacobi.pop().unwrap());

    // let mut gauss_seidel =
    //     GaussSeidel::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    // let mut data_gauss_seidel = gauss_seidel.solve(10e-10, 10_000);
    // println!("{}", gauss_seidel);
    // println!("{:?}", data_gauss_seidel.pop().unwrap());

    // let mut sor = SOR::<f64>::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1), 1.8);
    // let mut data_sor = sor.solve(10e-10, 10_000);
    // println!("{}", sor);
    // println!("{:?}", data_sor.pop().unwrap());

    //  let mut fg = Figure::new();

    //  let x_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.0).collect();
    //  let y_jacobi: Vec<f64> = data_jacobi.iter().map(|e| e.1).collect();
    //  let x_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.0).collect();
    //  let y_gause_seidel: Vec<f64> = data_gauss_seidel.iter().map(|e| e.1).collect();
    //  let x_sor: Vec<f64> = data_sor.iter().map(|e| e.0).collect();
    //  let y_sor: Vec<f64> = data_sor.iter().map(|e| e.1).collect();
    //  {
    //      let _axec = fg
    //          .axes2d()
    //          .set_x_axis(true, &[])
    //          .set_x_range(Fix(0.0), Fix(10000.0))
    //          .set_y_range(Fix(20e-6), Fix(4000000.0))
    //          .set_y_log(Some(10.0))
    //          .set_x_label("times", &[])
    //          .set_y_label("residual\\_norm", &[])
    //          .lines(x_jacobi, y_jacobi, &[Caption("Jacobi"), Color("green")])
    //          .lines(
    //              x_gause_seidel,
    //              y_gause_seidel,
    //              &[Caption("GaussSeidel"), Color("red")],
    //          )
    //          .lines(x_sor, y_sor, &[Caption("SOR"), Color("blue")]);
    //  }
    //  let _ = fg.show();

    // println!(
    //     "{}",
    //     (&(&matrixa.diagonal_matrix_inverse()
    //         * &(&matrixa.lower_triangular_matrix() + &matrixa.upper_triangular_matrix()))
    //         * (-1.0))
    //         .power_method(10000)
    //         .pop()
    //         .unwrap()
    //         .1
    // );

    let matrixa = data::matrix21a_f64();
    let matrixb = data::matrix21b_f64();
    let mut cg = CG::new(matrixa.clone(), matrixb.clone(), Matrix::new(9, 1));
    let mut data_cg = cg.solve(10e-10, 10_000);
    println!("{}", cg);
    println!("{:?}", data_cg.pop().unwrap());
    // println!("{}", matrixa.to_transpose());
}
